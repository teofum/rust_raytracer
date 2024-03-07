use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use rand_pcg::Pcg64Mcg;
use regex::Regex;

use crate::{
    camera::Camera,
    material::{Dielectric, Emissive, Glossy, LambertianDiffuse, Material, Metal},
    object::{obj_box, BoundingVolumeHierarchyNode, Plane, Sky, Sphere, Sun, Transform},
    texture::{
        CheckerboardSolidTexture, CheckerboardTexture, ConstantTexture, ImageTexture, Interpolate,
        Sampler, TexturePointer, UvDebugTexture,
    },
    utils::{deg_to_rad, ParseError},
    vec4::Color,
};
use crate::{
    config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG},
    object::Hit,
};
use crate::{object::ObjectList, utils::parse_vec};
use crate::{scene::SceneData, vec4::Vec4};

use super::obj::load_mesh_from_file;

enum Entity {
    Object(Arc<dyn Hit>),
    Material(Arc<dyn Material>),
    TextureColor(TexturePointer<Vec4>),
    TextureFloat(TexturePointer<f64>),
}

enum Texture {
    Color(TexturePointer<Vec4>),
    Float(TexturePointer<f64>),
}

type ParseResult = Result<Entity, Box<dyn Error>>;

pub struct SceneLoader<'a> {
    objects: HashMap<String, Arc<dyn Hit>>,
    materials: HashMap<String, Arc<dyn Material>>,
    color_textures: HashMap<String, Arc<dyn Sampler<Output = Vec4>>>,
    float_textures: HashMap<String, Arc<dyn Sampler<Output = f64>>>,

    scene_config: SceneConfig,
    asset_path: String,

    rng: &'a mut Pcg64Mcg,
}

impl<'a> SceneLoader<'a> {
    pub fn new(rng: &'a mut Pcg64Mcg, asset_path: &str) -> Self {
        SceneLoader {
            objects: HashMap::new(),
            materials: HashMap::new(),
            color_textures: HashMap::new(),
            float_textures: HashMap::new(),

            scene_config: DEFAULT_SCENE_CONFIG,
            asset_path: asset_path.to_owned(),

            rng,
        }
    }

    pub fn load(mut self, file: &File, config: Config) -> Result<SceneData, Box<dyn Error>> {
        let reader = BufReader::new(file);

        for (line_number, line) in reader.lines().flatten().enumerate() {
            if line.len() == 0 || line.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            if line.starts_with('@') {
                if let Some((directive, content)) = line.split_once(' ') {
                    match &directive[1..] {
                        "config" => {
                            let res = self.parse_config_directive(content);
                            if let Err(err) = res {
                                println!("Warning: invalid @config directive");
                                println!("\t{err}\n");
                            }
                        }
                        _ => (),
                    }
                }
                continue;
            }

            let mut line_parts = line.split(":").map(|s| s.trim());
            if let (Some(label), Some(decl)) = (line_parts.next(), line_parts.next()) {
                match self.parse_declaration(decl) {
                    Ok(entity) => {
                        let label = label.to_owned();
                        match entity {
                            Entity::Object(obj) => {
                                self.objects.insert(label, obj);
                            }
                            Entity::Material(mat) => {
                                self.materials.insert(label, mat);
                            }
                            Entity::TextureColor(tex) => {
                                self.color_textures.insert(label, tex);
                            }
                            Entity::TextureFloat(tex) => {
                                self.float_textures.insert(label, tex);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Warning: error on line {line_number}, skipped");
                        println!("\t{err}\n");
                    }
                }
            } else {
                println!("Warning: parse failed on line {line_number}, skipped");
                println!("\t{line}\n");
            }
        }

        if let (Some(world_ref), Some(lights_ref)) =
            (self.objects.get("world"), self.objects.get("lights"))
        {
            let world = Arc::clone(world_ref);
            let lights = Arc::clone(lights_ref);

            let scene_config = SceneConfig::merge(&self.scene_config, &config.scene);
            let config = Config {
                scene: scene_config,
                ..config
            };

            let camera = Camera::new(&config);

            Ok((camera, world, lights))
        } else {
            Err(Box::new(ParseError::new("No world/lights object")))
        }
    }

    fn parse_config_directive(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        if let Some((key, value)) = content.split_once('=') {
            let value = value.trim();
            match key.trim() {
                "output_width" => {
                    let w = value.parse::<usize>()?;
                    self.scene_config.output_width = Some(w);
                }
                "aspect_ratio" => {
                    let ratio = if value.contains('/') {
                        if let Some((a, b)) = value.split_once('/') {
                            let a = a.trim().parse::<f64>()?;
                            let b = b.trim().parse::<f64>()?;

                            a / b
                        } else {
                            return Err(Box::new(ParseError::new(
                                "Aspect ratio must be a number or division",
                            )));
                        }
                    } else {
                        value.parse::<f64>()?
                    };
                    self.scene_config.aspect_ratio = Some(ratio);
                }
                "focal_length" => {
                    let f = value.parse::<f64>()?;
                    self.scene_config.focal_length = Some(f);
                }
                "f_number" => {
                    let f = value.parse::<f64>()?;
                    self.scene_config.f_number = Some(f);
                }
                "focus_distance" => {
                    let d = value.parse::<f64>()?;
                    self.scene_config.focus_distance = Some(d);
                }
                "camera_pos" => {
                    let [x, y, z] = parse_vec(value)?;
                    let vec = Vec4::point(x, y, z);
                    self.scene_config.camera_pos = Some(vec);
                }
                "camera_target" => {
                    let [x, y, z] = parse_vec(value)?;
                    let vec = Vec4::point(x, y, z);
                    self.scene_config.camera_target = Some(vec);
                }
                _ => (),
            };

            Ok(())
        } else {
            Err(Box::new(ParseError::new(&format!("@config {content}"))))
        }
    }

    fn parse_params(&self, decl: &str) -> Vec<String> {
        let mut params = Vec::new();
        let mut current = String::new();
        let mut nest_level = 0;

        // Assumes the declaration doesn't contain multi-codepoint Unicode
        for char in decl.chars() {
            match char {
                '(' => {
                    current.push('(');
                    nest_level += 1;
                }
                ')' => {
                    current.push(')');
                    nest_level -= 1;
                }
                ' ' => {
                    if nest_level > 0 {
                        current.push(' '); // Inside parentheses, treat a space like any other char
                    } else {
                        // At root level spaces separate parameters
                        params.push(current);
                        current = String::new();
                    }
                }
                c => current.push(c),
            }
        }
        params.push(current);

        params
    }

    fn parse_declaration(&mut self, decl: &str) -> ParseResult {
        let mut params = self.parse_params(decl).into_iter();

        if let Some(item_type) = params.next() {
            match &item_type[..] {
                // Textures
                "constant" => self.create_constant_tex(&mut params),
                "checker" => self.create_checker_tex(&mut params, false),
                "checker_solid" => self.create_checker_tex(&mut params, true),
                "lerp" => self.create_lerp_tex(&mut params),
                "noise" => Err(Box::new(ParseError::new("Not implemented"))),
                "noise_solid" => Err(Box::new(ParseError::new("Not implemented"))),
                "image" => self.create_image_tex(&mut params),
                "uv_debug" => Ok(Entity::TextureColor(Arc::new(UvDebugTexture))),
                // Materials
                "lambertian" => self.create_lambertian(&mut params),
                "metal" => self.create_metal(&mut params),
                "glass" => self.create_dielectric(&mut params),
                "glossy" => self.create_glossy(&mut params),
                "emissive" => self.create_emissive(&mut params),
                "isotropic" => Err(Box::new(ParseError::new("Not implemented"))),
                // Objects
                "sphere" => self.create_sphere(&mut params),
                "plane" => self.create_plane(&mut params),
                "box" => self.create_box(&mut params),
                "mesh" => self.create_mesh(&mut params),
                "transform" => self.create_transform(&mut params),
                "list" => self.create_list(&mut params),
                "bvh" => self.create_bvh(&mut params),
                "sky" => self.create_sky(&mut params),
                "sun" => self.create_sun(&mut params),
                "volume" => Err(Box::new(ParseError::new("Not implemented"))),
                _ => Err(Box::new(ParseError::new("Unknown object type"))),
            }
        } else {
            Err(Box::new(ParseError::new(
                "Constant texture missing parameters",
            )))
        }
    }

    /// Get a color (Vec4) texture from either a reference or inline declaration
    fn get_color_texture(&mut self, expr: &str) -> Result<TexturePointer<Vec4>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.color_textures.get(label) {
                Some(tex) => Ok(Arc::clone(tex)),
                None => {
                    let err_str = format!("Invalid texture reference {}", label);
                    Err(Box::new(ParseError::new(&err_str)))
                }
            }
        } else if is_inline {
            let decl = &expr[1..(expr.len() - 1)];
            match self.parse_declaration(decl) {
                Ok(Entity::TextureColor(tex)) => Ok(tex),
                Ok(_) => Err(Box::new(ParseError::new(
                    "Expression evaluates to a different entity type, expected color texture",
                ))),
                Err(err) => Err(err),
            }
        } else {
            Err(Box::new(ParseError::new(
                "Expected a reference or inline declaration",
            )))
        }
    }

    /// Get a float texture from either a reference or inline declaration
    fn get_float_texture(&mut self, expr: &str) -> Result<TexturePointer<f64>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.float_textures.get(label) {
                Some(tex) => Ok(Arc::clone(tex)),
                None => {
                    let err_str = format!("Invalid texture reference {}", label);
                    Err(Box::new(ParseError::new(&err_str)))
                }
            }
        } else if is_inline {
            let decl = &expr[1..(expr.len() - 1)];
            match self.parse_declaration(decl) {
                Ok(Entity::TextureFloat(tex)) => Ok(tex),
                Ok(_) => Err(Box::new(ParseError::new(
                    "Expression evaluates to a different entity type, expected float texture",
                ))),
                Err(err) => Err(err),
            }
        } else {
            Err(Box::new(ParseError::new(
                "Expected a reference or inline declaration",
            )))
        }
    }

    fn get_texture(&mut self, expr: &str) -> Result<Texture, Box<dyn Error>> {
        // First try to get a vec texture
        match self.get_color_texture(expr) {
            Ok(tex) => Ok(Texture::Color(tex)),
            Err(_) => {
                // Try to get a color texture
                let tex = self.get_float_texture(expr)?;
                Ok(Texture::Float(tex))
            }
        }
    }

    /// Get a material from either a reference or inline declaration
    fn get_material(&mut self, expr: &str) -> Result<Arc<dyn Material>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.materials.get(label) {
                Some(mat) => Ok(Arc::clone(mat)),
                None => {
                    let err_str = format!("Invalid material reference {}", label);
                    Err(Box::new(ParseError::new(&err_str)))
                }
            }
        } else if is_inline {
            let decl = &expr[1..(expr.len() - 1)];
            match self.parse_declaration(decl) {
                Ok(Entity::Material(mat)) => Ok(mat),
                Ok(_) => Err(Box::new(ParseError::new(
                    "Expression evaluates to a different entity type, expected material",
                ))),
                Err(err) => Err(err),
            }
        } else {
            Err(Box::new(ParseError::new(
                "Expected a reference or inline declaration",
            )))
        }
    }

    /// Get an object from either a reference or inline declaration
    fn get_object(&mut self, expr: &str) -> Result<Arc<dyn Hit>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.objects.get(label) {
                Some(obj) => Ok(Arc::clone(obj)),
                None => {
                    let err_str = format!("Invalid object reference {}", label);
                    Err(Box::new(ParseError::new(&err_str)))
                }
            }
        } else if is_inline {
            let decl = &expr[1..(expr.len() - 1)];
            match self.parse_declaration(decl) {
                Ok(Entity::Object(obj)) => Ok(obj),
                Ok(_) => Err(Box::new(ParseError::new(
                    "Expression evaluates to a different entity type, expected object",
                ))),
                Err(err) => Err(err),
            }
        } else {
            Err(Box::new(ParseError::new(
                "Expected a reference or inline declaration",
            )))
        }
    }

    // =========================================================================
    // Textures
    // =========================================================================

    fn create_constant_tex(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(value) = params.next() {
            if let Ok([x, y, z]) = parse_vec(&value) {
                let vec = Vec4::vec(x, y, z);
                let texture = ConstantTexture::new(vec);

                Ok(Entity::TextureColor(Arc::new(texture)))
            } else {
                match value.parse::<f64>() {
                    Ok(k) => {
                        let texture = ConstantTexture::new(k);
                        Ok(Entity::TextureFloat(Arc::new(texture)))
                    }
                    Err(err) => Err(Box::new(err)),
                }
            }
        } else {
            Err(Box::new(ParseError::new(
                "Constant texture missing parameters",
            )))
        }
    }

    fn create_checker_tex(
        &mut self,
        params: &mut dyn Iterator<Item = String>,
        solid: bool,
    ) -> ParseResult {
        if let (Some(tex1_expr), Some(tex2_expr)) = (params.next(), params.next()) {
            let scale = params.next().map_or(1.0, |s| s.parse::<f64>().unwrap());

            let tex1 = self.get_texture(&tex1_expr)?;
            match tex1 {
                Texture::Color(tex1) => {
                    let tex2 = self.get_color_texture(&tex2_expr)?;
                    let texture: Arc<dyn Sampler<Output = Color>> = if solid {
                        Arc::new(CheckerboardSolidTexture::new(tex1, tex2, scale))
                    } else {
                        Arc::new(CheckerboardTexture::new(tex1, tex2, scale))
                    };
                    Ok(Entity::TextureColor(texture))
                }
                Texture::Float(tex1) => {
                    let tex2 = self.get_float_texture(&tex2_expr)?;
                    let texture: Arc<dyn Sampler<Output = f64>> = if solid {
                        Arc::new(CheckerboardSolidTexture::new(tex1, tex2, scale))
                    } else {
                        Arc::new(CheckerboardTexture::new(tex1, tex2, scale))
                    };
                    Ok(Entity::TextureFloat(texture))
                }
            }
        } else {
            Err(Box::new(ParseError::new(
                "Checkerboard texture missing parameters",
            )))
        }
    }

    fn create_lerp_tex(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(tex1_expr), Some(tex2_expr), Some(t_expr)) =
            (params.next(), params.next(), params.next())
        {
            let t = self.get_float_texture(&t_expr)?;
            let tex1 = self.get_texture(&tex1_expr)?;
            match tex1 {
                Texture::Color(tex1) => {
                    let tex2 = self.get_color_texture(&tex2_expr)?;
                    let texture = Interpolate::new(tex1, tex2, t);
                    Ok(Entity::TextureColor(Arc::new(texture)))
                }
                Texture::Float(tex1) => {
                    let tex2 = self.get_float_texture(&tex2_expr)?;
                    let texture = Interpolate::new(tex1, tex2, t);
                    Ok(Entity::TextureFloat(Arc::new(texture)))
                }
            }
        } else {
            Err(Box::new(ParseError::new(
                "Interpolate texture missing parameters",
            )))
        }
    }

    fn create_image_tex(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(file_path) = params.next() {
            let path = &(self.asset_path.to_owned() + &file_path);
            let texture = ImageTexture::from_file(path)?;

            Ok(Entity::TextureColor(Arc::new(texture)))
        } else {
            Err(Box::new(ParseError::new(
                "Image texture missing parameters",
            )))
        }
    }

    // =========================================================================
    // Materials
    // =========================================================================

    fn create_lambertian(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(albedo_expr) = params.next() {
            let albedo = self.get_color_texture(&albedo_expr)?;
            let material = LambertianDiffuse::new(albedo);
            Ok(Entity::Material(Arc::new(material)))
        } else {
            Err(Box::new(ParseError::new(
                "LambertianDiffuse material missing parameters",
            )))
        }
    }

    fn create_metal(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(albedo_expr), Some(rough_expr)) = (params.next(), params.next()) {
            let albedo = self.get_color_texture(&albedo_expr)?;
            let roughness = self.get_float_texture(&rough_expr)?;
            let material = Metal::new(albedo, roughness);
            Ok(Entity::Material(Arc::new(material)))
        } else {
            Err(Box::new(ParseError::new(
                "Metal material missing parameters",
            )))
        }
    }

    fn create_dielectric(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        let ior = params.next().map_or(1.5, |ior| ior.parse::<f64>().unwrap());
        let material = Dielectric::new(ior);
        Ok(Entity::Material(Arc::new(material)))
    }

    fn create_glossy(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(albedo_expr), Some(rough_expr)) = (params.next(), params.next()) {
            let albedo = self.get_color_texture(&albedo_expr)?;
            let roughness = self.get_float_texture(&rough_expr)?;
            let ior = params.next().map_or(1.5, |ior| ior.parse::<f64>().unwrap());
            let material = Glossy::new(albedo, roughness, ior);
            Ok(Entity::Material(Arc::new(material)))
        } else {
            Err(Box::new(ParseError::new(
                "Glossy material missing parameters",
            )))
        }
    }

    fn create_emissive(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(expr) = params.next() {
            let texture = self.get_color_texture(&expr)?;
            let material = Emissive::new(texture);
            Ok(Entity::Material(Arc::new(material)))
        } else {
            Err(Box::new(ParseError::new(
                "Emissive material missing parameters",
            )))
        }
    }

    // =========================================================================
    // Objects
    // =========================================================================

    fn create_sphere(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(origin), Some(radius), Some(mat_expr)) =
            (params.next(), params.next(), params.next())
        {
            let [x, y, z] = parse_vec(&origin)?;
            let origin = Vec4::point(x, y, z);
            let radius = radius.parse::<f64>()?;

            let material = self.get_material(&mat_expr)?;

            let sphere = Sphere::new(origin, radius, material);
            Ok(Entity::Object(Arc::new(sphere)))
        } else {
            Err(Box::new(ParseError::new("Sphere missing parameters")))
        }
    }

    fn create_plane(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(origin), Some(u), Some(v), Some(mat_expr)) =
            (params.next(), params.next(), params.next(), params.next())
        {
            let [x, y, z] = parse_vec(&origin)?;
            let origin = Vec4::point(x, y, z);
            let [ux, uy, uz] = parse_vec(&u)?;
            let u = Vec4::vec(ux, uy, uz);
            let [vx, vy, vz] = parse_vec(&v)?;
            let v = Vec4::vec(vx, vy, vz);

            let material = self.get_material(&mat_expr)?;

            let mut plane = Plane::new(origin, (u, v), material);

            if let Some(p) = params.next() {
                if p == "backface" {
                    plane.render_backface = true;
                }
            }

            Ok(Entity::Object(Arc::new(plane)))
        } else {
            Err(Box::new(ParseError::new("Plane missing parameters")))
        }
    }

    fn create_box(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(origin), Some(size), Some(mat_expr)) =
            (params.next(), params.next(), params.next())
        {
            let [x, y, z] = parse_vec(&origin)?;
            let origin = Vec4::point(x, y, z);
            let [sx, sy, sz] = parse_vec(&size)?;
            let size = Vec4::point(sx, sy, sz);

            let material = self.get_material(&mat_expr)?;

            let box_obj = obj_box::make_box(origin, size, material);
            Ok(Entity::Object(Arc::new(box_obj)))
        } else {
            Err(Box::new(ParseError::new("Box missing parameters")))
        }
    }

    fn create_mesh(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(file_path), Some(mat_expr)) = (params.next(), params.next()) {
            let path = &(self.asset_path.to_owned() + &file_path);
            let file = File::open(path)?;
            let material = self.get_material(&mat_expr)?;

            let mesh = load_mesh_from_file(&file, material)?;
            Ok(Entity::Object(Arc::new(mesh)))
        } else {
            Err(Box::new(ParseError::new("Mesh missing parameters")))
        }
    }

    fn create_transform(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        let param_regex = Regex::new(r"^([^=\s]+)=([^=\s]+)$").unwrap();

        if let Some(obj_expr) = params.next() {
            let object = self.get_object(&obj_expr)?;
            let mut transform = Transform::new(object);

            while let Some(param) = params.next() {
                for (_, [key, value]) in param_regex.captures_iter(&param).map(|c| c.extract()) {
                    match key {
                        "t" => {
                            let vec = parse_vec(value)?;
                            transform.translate(vec[0], vec[1], vec[2]);
                        }
                        "s" => {
                            if let Ok(vec) = parse_vec(value) {
                                transform.scale(vec[0], vec[1], vec[2]);
                            } else {
                                let s = value.parse::<f64>()?;
                                transform.scale_uniform(s);
                            }
                        }
                        "rx" => {
                            let deg = value.parse::<f64>()?;
                            transform.rotate_x(deg_to_rad(deg));
                        }
                        "ry" => {
                            let deg = value.parse::<f64>()?;
                            transform.rotate_y(deg_to_rad(deg));
                        }
                        "rz" => {
                            let deg = value.parse::<f64>()?;
                            transform.rotate_z(deg_to_rad(deg));
                        }
                        _ => (),
                    }
                }
            }

            Ok(Entity::Object(Arc::new(transform)))
        } else {
            Err(Box::new(ParseError::new("Transform missing parameters")))
        }
    }

    fn create_list(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        let mut list = ObjectList::new();

        while let Some(obj_expr) = params.next() {
            let obj = self.get_object(&obj_expr)?;
            list.add(obj);
        }

        Ok(Entity::Object(Arc::new(list)))
    }

    fn create_bvh(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(axes) = params.next() {
            let axes = [axes.contains('x'), axes.contains('y'), axes.contains('z')];
            let mut objs = Vec::new();

            while let Some(obj_expr) = params.next() {
                let obj = self.get_object(&obj_expr)?;
                objs.push(obj);
            }

            let bvh = BoundingVolumeHierarchyNode::from(objs, axes, self.rng);

            Ok(Entity::Object(Arc::new(bvh)))
        } else {
            Err(Box::new(ParseError::new("BVH missing parameters")))
        }
    }

    fn create_sky(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(tex_expr) = params.next() {
            let texture = self.get_color_texture(&tex_expr)?;
            let sky = Sky::new(texture);

            Ok(Entity::Object(Arc::new(sky)))
        } else {
            Err(Box::new(ParseError::new("Sky missing parameters")))
        }
    }

    fn create_sun(&mut self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(dir), Some(tex_expr)) = (params.next(), params.next()) {
            let [x, y, z] = parse_vec(&dir)?;
            let dir = Vec4::point(x, y, z);

            let texture = self.get_color_texture(&tex_expr)?;
            let sun = Sun::new(texture, dir);

            Ok(Entity::Object(Arc::new(sun)))
        } else {
            Err(Box::new(ParseError::new("Sun missing parameters")))
        }
    }
}
