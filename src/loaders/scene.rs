use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use crate::{
    camera::Camera,
    material::{Emissive, Glossy, Material},
    object::{Plane, Sky, Sphere},
    texture::{ConstantTexture, Sampler, TexturePointer},
    utils::ParseError,
};
use crate::{
    config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG},
    object::Hit,
};
use crate::{object::ObjectList, utils::parse_vec};
use crate::{scene::SceneData, vec4::Vec4};

enum Entity {
    Object(Arc<dyn Hit>),
    Material(Arc<dyn Material>),
    TextureColor(TexturePointer<Vec4>),
    TextureFloat(TexturePointer<f64>),
}

type ParseResult = Result<Entity, Box<dyn Error>>;

pub struct SceneLoader {
    objects: HashMap<String, Arc<dyn Hit>>,
    materials: HashMap<String, Arc<dyn Material>>,
    color_textures: HashMap<String, Arc<dyn Sampler<Output = Vec4>>>,
    float_textures: HashMap<String, Arc<dyn Sampler<Output = f64>>>,
}

impl SceneLoader {
    pub fn new() -> Self {
        SceneLoader {
            objects: HashMap::new(),
            materials: HashMap::new(),
            color_textures: HashMap::new(),
            float_textures: HashMap::new(),
        }
    }

    pub fn load(mut self, file: &File, config: Config) -> Result<SceneData, Box<dyn Error>> {
        let reader = BufReader::new(file);

        let scene_config = SceneConfig::merge(&DEFAULT_SCENE_CONFIG, &config.scene);
        let config = Config {
            scene: scene_config,
            ..config
        };

        let camera = Camera::new(&config);

        for (line_number, line) in reader.lines().flatten().enumerate() {
            if line.len() == 0 || line.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            let mut line_parts = line.split(":").map(|s| s.trim());
            if let (Some(label), Some(decl)) = (line_parts.next(), line_parts.next()) {
                println!("{:4} {label}: {decl}", line_number);

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
            Ok((camera, world, lights))
        } else {
            Err(Box::new(ParseError::new("No world/lights object")))
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

    fn parse_declaration(&self, decl: &str) -> ParseResult {
        let mut params = self.parse_params(decl).into_iter();

        if let Some(item_type) = params.next() {
            match &item_type[..] {
                // Textures
                "constant" => self.create_constant_tex(&mut params),
                "checker" => Err(Box::new(ParseError::new("Not implemented"))),
                "checker_solid" => Err(Box::new(ParseError::new("Not implemented"))),
                "lerp" => Err(Box::new(ParseError::new("Not implemented"))),
                "noise" => Err(Box::new(ParseError::new("Not implemented"))),
                "noise_solid" => Err(Box::new(ParseError::new("Not implemented"))),
                "image" => Err(Box::new(ParseError::new("Not implemented"))),
                "uv_debug" => Err(Box::new(ParseError::new("Not implemented"))),
                // Materials
                "lambertian" => Err(Box::new(ParseError::new("Not implemented"))),
                "metal" => Err(Box::new(ParseError::new("Not implemented"))),
                "glass" => Err(Box::new(ParseError::new("Not implemented"))),
                "glossy" => self.create_glossy(&mut params),
                "emissive" => self.create_emissive(&mut params),
                "isotropic" => Err(Box::new(ParseError::new("Not implemented"))),
                // Objects
                "sphere" => self.create_sphere(&mut params),
                "plane" => self.create_plane(&mut params),
                "box" => Err(Box::new(ParseError::new("Not implemented"))),
                "mesh" => Err(Box::new(ParseError::new("Not implemented"))),
                "transform" => Err(Box::new(ParseError::new("Not implemented"))),
                "list" => self.create_list(&mut params),
                "bvh" => Err(Box::new(ParseError::new("Not implemented"))),
                "sky" => self.create_sky(&mut params),
                "sun" => Err(Box::new(ParseError::new("Not implemented"))),
                "volume" => Err(Box::new(ParseError::new("Not implemented"))),
                _ => {
                    return Err(Box::new(ParseError::new("Unknown object type")));
                }
            }
        } else {
            Err(Box::new(ParseError::new(
                "Constant texture missing parameters",
            )))
        }
    }

    /// Get a color (Vec4) texture from either a reference or inline declaration
    fn get_color_texture(&self, expr: &str) -> Result<TexturePointer<Vec4>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.color_textures.get(label) {
                Some(tex) => Ok(Arc::clone(tex)),
                None => {
                    let err_str = format!("Invalid color texture reference {}", label);
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
    fn get_float_texture(&self, expr: &str) -> Result<TexturePointer<f64>, Box<dyn Error>> {
        let is_reference = expr.starts_with('$');
        let is_inline = expr.starts_with('(') && expr.ends_with(')');

        if is_reference {
            let label = &expr[1..];
            match self.float_textures.get(label) {
                Some(tex) => Ok(Arc::clone(tex)),
                None => {
                    let err_str = format!("Invalid float texture reference {}", label);
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

    /// Get a material from either a reference or inline declaration
    fn get_material(&self, expr: &str) -> Result<Arc<dyn Material>, Box<dyn Error>> {
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
    fn get_object(&self, expr: &str) -> Result<Arc<dyn Hit>, Box<dyn Error>> {
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

    // =========================================================================
    // Materials
    // =========================================================================

    fn create_emissive(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
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

    fn create_glossy(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(albedo_expr), Some(rough_expr), Some(ior)) =
            (params.next(), params.next(), params.next())
        {
            let albedo = self.get_color_texture(&albedo_expr)?;
            let roughness = self.get_float_texture(&rough_expr)?;
            let ior = ior.parse::<f64>()?;
            let material = Glossy::new(albedo, roughness, ior);
            Ok(Entity::Material(Arc::new(material)))
        } else {
            Err(Box::new(ParseError::new(
                "Glossy material missing parameters",
            )))
        }
    }

    // =========================================================================
    // Objects
    // =========================================================================

    fn create_sphere(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(origin), Some(radius), Some(material)) =
            (params.next(), params.next(), params.next())
        {
            let [x, y, z] = parse_vec(&origin)?;
            let origin = Vec4::point(x, y, z);
            let radius = radius.parse::<f64>()?;

            // TODO get material
            let material = self.get_material(&material)?;

            let sphere = Sphere::new(origin, radius, material);
            Ok(Entity::Object(Arc::new(sphere)))
        } else {
            Err(Box::new(ParseError::new("Sphere missing parameters")))
        }
    }

    fn create_plane(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let (Some(origin), Some(u), Some(v), Some(material)) =
            (params.next(), params.next(), params.next(), params.next())
        {
            let [x, y, z] = parse_vec(&origin)?;
            let origin = Vec4::point(x, y, z);
            let [ux, uy, uz] = parse_vec(&u)?;
            let u = Vec4::point(ux, uy, uz);
            let [vx, vy, vz] = parse_vec(&v)?;
            let v = Vec4::point(vx, vy, vz);

            // TODO get material
            let material = self.get_material(&material)?;

            let plane = Plane::new(origin, (u, v), material);
            Ok(Entity::Object(Arc::new(plane)))
        } else {
            Err(Box::new(ParseError::new("Sphere missing parameters")))
        }
    }

    fn create_list(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        let mut list = ObjectList::new();

        while let Some(expr) = params.next() {
            let obj = self.get_object(&expr)?;
            list.add(obj);
        }

        Ok(Entity::Object(Arc::new(list)))
    }

    fn create_sky(&self, params: &mut dyn Iterator<Item = String>) -> ParseResult {
        if let Some(expr) = params.next() {
            let texture = self.get_color_texture(&expr)?;
            let sky = Sky::new(texture);

            Ok(Entity::Object(Arc::new(sky)))
        } else {
            Err(Box::new(ParseError::new("Sky missing parameters")))
        }
    }
}
