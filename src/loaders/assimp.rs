use std::error::Error;
use std::rc::Rc;
use std::sync::Arc;

use rand_distr::num_traits::ToPrimitive;
use russimp::material::{Material as AssimpMaterial, PropertyTypeInfo};
use russimp::mesh::Mesh as AssimpMesh;
use russimp::node::Node;
use russimp::scene::{PostProcess, Scene};

use crate::camera::Camera;
use crate::config::{Config, DEFAULT_SCENE_CONFIG, SceneConfig};
use crate::mat4::Mat4;
use crate::material::{Emissive, Glossy, LambertianDiffuse};
use crate::material::Material;
use crate::object::{Hit, ObjectList, Sphere, Transform};
use crate::object::mesh::{Triangle, TriangleMesh};
use crate::scene::SceneData;
use crate::texture::ConstantTexture;
use crate::utils::ParseError;
use crate::vec4::{Point4, Vec4};

pub struct AssimpLoader {
    scene: Scene,
}

impl AssimpLoader {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let scene = Scene::from_file(file_path, vec![
            PostProcess::Triangulate
        ])?;
        Ok(AssimpLoader { scene })
    }

    pub fn load(&self, config: Config) -> Result<SceneData, Box<dyn Error>> {
        if let Some(root_node) = &self.scene.root {
            let mut scene_config = DEFAULT_SCENE_CONFIG;

            if let Some(camera) = self.scene.cameras.get(0) {
                let c_pos = camera.position;
                let c_look = camera.look_at;
                let hfov = camera.horizontal_fov as f64;

                scene_config.camera_pos = Some(Vec4::point(c_pos.x as f64, c_pos.y as f64, c_pos.z as f64));
                scene_config.camera_target = Some(Vec4::point(c_look.x as f64, c_look.y as f64, c_look.z as f64));
                scene_config.aspect_ratio = Some(camera.aspect as f64);
                scene_config.focal_length = Some(18.0 / f64::tan(hfov / 2.0));
            }

            let scene_config = SceneConfig::merge(&scene_config, &config.scene);
            let config = Config {
                scene: scene_config,
                ..config
            };


            let camera = Camera::new(&config);

            let mut lights: Vec<Arc<dyn Hit>> = Vec::new();
            let world = self.load_node(root_node, Vec4::point(0.0, 0.0, 0.0), &mut lights);
            let lights: Arc<dyn Hit> = Arc::new(ObjectList::from(lights));
            Ok((camera, Arc::clone(&world), lights))
        } else {
            Err(Box::new(ParseError::new("Assimp load fail")))
        }
    }

    fn load_node(&self, node: &Rc<Node>, pos: Point4, lights: &mut Vec<Arc<dyn Hit>>) -> Arc<dyn Hit> {
        let children = node.children.borrow();
        let mut objects = Vec::with_capacity(node.meshes.len() + children.len());

        let t_mat = Mat4::from_assimp(&node.transformation);
        let mut translation = t_mat.column(3);
        translation[3] = 0.0;

        for mesh_idx in &node.meshes {
            let mesh = self.load_mesh(&self.scene.meshes[mesh_idx.to_usize().unwrap()], pos + translation, lights);
            objects.push(mesh);
        }

        for child in children.iter() {
            objects.push(self.load_node(child, pos + translation, lights));
        }

        let list = ObjectList::from(objects);
        let transform = Transform::from_matrix(Arc::new(list), &t_mat);
        Arc::new(transform)
    }

    fn load_mesh(&self, mesh: &AssimpMesh, pos: Point4, lights: &mut Vec<Arc<dyn Hit>>) -> Arc<dyn Hit> {
        let vertices: Vec<Vec4> = mesh.vertices.iter().map(|v| {
            Vec4::point(v.x as f64, v.y as f64, v.z as f64)
        }).collect();
        let normals = mesh.normals.iter().map(|n| {
            Vec4::vec(n.x as f64, n.y as f64, n.z as f64)
        }).collect();
        let uvs = if let Some(Some(mesh_uvs)) = &mesh.texture_coords.get(0) {
            mesh_uvs.iter().map(|uv| {
                Vec4::vec(uv.x as f64, uv.y as f64, uv.z as f64)
            }).collect()
        } else {
            vec![]
        };

        let tris = mesh.faces.iter().map(|face| {
            let indices: [usize; 3] = [
                face.0[0] as usize,
                face.0[1] as usize,
                face.0[2] as usize,
            ];
            Triangle {
                vert_indices: indices,
                normal_indices: indices,
                uv_indices: if uvs.len() > 0 { Some(indices) } else { None },
            }
        }).collect();

        let (material, is_emissive) = self.load_material(&self.scene.materials[mesh.material_index as usize]);

        if is_emissive {
            // Create an invisible sphere object to sample lighting
            // This is a bit of a hack, but I don't support importance sampling from arbitrary meshes
            let radius = vertices.iter().fold(f64::INFINITY, |r, v| f64::min(v.length(), r));
            let sampler = Sphere::new(pos, radius, Arc::clone(&material));
            lights.push(Arc::new(sampler))
        }
        Arc::new(TriangleMesh::new(vertices, uvs, normals, tris, material))
    }

    fn load_material(&self, mat: &AssimpMaterial) -> (Arc<dyn Material>, bool) {
        // Emissive material
        if let Some(mut emission) = get_vec3_property(mat, "$clr.emissive") {
            if emission.length_squared() > 0.0 {
                let intensity = get_float_property(mat, "$mat.emissiveIntensity").unwrap_or(1.0);
                emission *= intensity;

                return (Arc::new(Emissive::new(Arc::new(ConstantTexture::new(emission)))), true);
            }
        }

        // Use glossy material for anything else
        // TODO: metals
        if let Some(albedo) = get_vec3_property(mat, "$clr.base") {
            let roughness = get_float_property(mat, "$mat.roughnessFactor").unwrap_or(0.0);
            let ior = 1.5;

            return (
                Arc::new(Glossy::new(
                    Arc::new(ConstantTexture::new(albedo)),
                    Arc::new(ConstantTexture::new(roughness)),
                    ior,
                )),
                false,
            );
        }

        (
            Arc::new(LambertianDiffuse::new(Arc::new(ConstantTexture::from_values(0.5, 0.5, 0.5)))),
            false,
        )
    }
}

fn get_vec3_property(mat: &AssimpMaterial, prop_name: &str) -> Option<Vec4> {
    if let Some(property) = mat.properties.iter().find(|p| p.key == prop_name) {
        let value = match &property.data {
            PropertyTypeInfo::FloatArray(arr) => Vec4::vec(arr[0] as f64, arr[1] as f64, arr[2] as f64),
            _ => panic!(),
        };
        Some(value)
    } else {
        None
    }
}

fn get_float_property(mat: &AssimpMaterial, prop_name: &str) -> Option<f64> {
    if let Some(property) = mat.properties.iter().find(|p| p.key == prop_name) {
        let value = match &property.data {
            PropertyTypeInfo::FloatArray(arr) => arr[0] as f64,
            _ => panic!(),
        };
        Some(value)
    } else {
        None
    }
}
