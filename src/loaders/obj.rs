use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    sync::Arc,
};

use crate::vec3::{Point3, Vec3};
use crate::{
    material::Material,
    object::mesh::{Triangle, TriangleMesh},
};

pub fn load_mesh_from_file(file: &File, material: Arc<dyn Material>) -> io::Result<TriangleMesh> {
    let reader = BufReader::new(file);

    let mut vertex_positions: Vec<Point3> = Vec::new();
    let mut vertex_uvs: Vec<Point3> = Vec::new();
    let mut vertex_normals: Vec<Vec3> = Vec::new();

    let mut triangles: Vec<Triangle> = Vec::new();

    // Parse OBJ file
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut params = line.split(' ');
            let cmd = params.next();

            match cmd {
                Some("v") => {
                    let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                    assert!(params.len() >= 3);

                    vertex_positions.push(Vec3(params[0], params[1], params[2]));
                }
                Some("vt") => {
                    let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                    assert!(params.len() >= 2);

                    vertex_uvs.push(Vec3(params[0], params[1], *params.get(2).unwrap_or(&0.0)));
                }
                Some("vn") => {
                    let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                    assert!(params.len() >= 3);

                    vertex_normals.push(Vec3(params[0], params[1], params[2]).to_unit());
                }
                Some("f") => {
                    let params = params.map(|p| {
                        let parts: Vec<_> = p
                            .split('/')
                            .map(|n| match n {
                                "" => None,
                                _ => Some(n.parse::<i32>().unwrap()),
                            })
                            .collect();
                        assert!(parts.len() >= 3, "Vertices without normals are unsupported");
                        parts
                    });

                    let mut i_vert: [usize; 3] = [0; 3];
                    let mut i_norm: [usize; 3] = [0; 3];
                    let i_uv: Option<[usize; 3]> = None;

                    for (i, param) in params.enumerate() {
                        i_vert[i] = match param.get(0).unwrap().unwrap() {
                            idx if idx > 0 => idx - 1,
                            idx if idx < 0 => vertex_positions.len() as i32 + idx,
                            _ => 0,
                        } as usize;
                        i_norm[i] = match param.get(2).unwrap().unwrap() {
                            idx if idx > 0 => idx - 1,
                            idx if idx < 0 => vertex_normals.len() as i32 + idx,
                            _ => 0,
                        } as usize;
                    }

                    triangles.push(Triangle {
                        vert_indices: i_vert,
                        normal_indices: i_norm,
                        uv_indices: i_uv,
                    })
                }
                Some(_) => continue,
                None => continue,
            }
        }
    }

    Ok(TriangleMesh::new(
        vertex_positions,
        vertex_uvs,
        vertex_normals,
        triangles,
        material,
    ))
}
