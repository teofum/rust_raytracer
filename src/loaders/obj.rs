use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    sync::Arc,
};

use crate::vec4::{Point4, Vec4};
use crate::{
    material::Material,
    object::mesh::{Triangle, TriangleMesh},
};

pub fn load_mesh_from_file(file: &File, material: Arc<dyn Material>) -> io::Result<TriangleMesh> {
    let reader = BufReader::new(file);

    let mut vertex_positions: Vec<Point4> = Vec::new();
    let mut vertex_uvs: Vec<Point4> = Vec::new();
    let mut vertex_normals: Vec<Vec4> = Vec::new();

    let mut triangles: Vec<Triangle> = Vec::new();

    // Parse OBJ file
    for line in reader.lines().flatten() {
        let mut params = line.split(' ');
        let cmd = params.next();

        match cmd {
            Some("v") => {
                let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                assert!(params.len() >= 3);

                vertex_positions.push(Vec4::point(params[0], params[1], params[2]));
            }
            Some("vt") => {
                let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                assert!(params.len() >= 2);

                vertex_uvs.push(Vec4::vec(
                    params[0],
                    params[1],
                    *params.get(2).unwrap_or(&0.0),
                ));
            }
            Some("vn") => {
                let params: Vec<_> = params.map(|p| p.parse::<f64>().unwrap()).collect();
                assert!(params.len() >= 3);

                vertex_normals.push(Vec4::vec(params[0], params[1], params[2]).to_unit());
            }
            Some("f") => {
                let params = params.map(|p| {
                    let parts = p.split('/').map(|n| match n {
                        "" => None,
                        _ => Some(n.parse::<i32>().unwrap()),
                    });
                    parts
                });

                let mut i_vert: [usize; 3] = [0; 3];
                let mut i_norm: [usize; 3] = [0; 3];
                let mut i_uv: [Option<usize>; 3] = [None; 3];

                for (i, mut param) in params.enumerate() {
                    i_vert[i] = match param.next().unwrap().unwrap() {
                        idx if idx > 0 => idx - 1,
                        idx if idx < 0 => vertex_positions.len() as i32 + idx,
                        _ => 0,
                    } as usize;
                    i_uv[i] = match param.next().unwrap() {
                        Some(idx) if idx > 0 => Some(idx as usize - 1),
                        Some(idx) if idx < 0 => Some((vertex_uvs.len() as i32 + idx) as usize),
                        Some(_) => Some(0),
                        None => None,
                    };
                    i_norm[i] = match param.next().unwrap().unwrap() {
                        idx if idx > 0 => idx - 1,
                        idx if idx < 0 => vertex_normals.len() as i32 + idx,
                        _ => 0,
                    } as usize;
                }

                let has_uvs = i_uv[0].is_some() && i_uv[1].is_some() && i_uv[2].is_some();

                triangles.push(Triangle {
                    vert_indices: i_vert,
                    normal_indices: i_norm,
                    uv_indices: if has_uvs {
                        Some(i_uv.map(|x| x.unwrap()))
                    } else {
                        None
                    },
                })
            }
            Some(_) => continue,
            None => continue,
        }
    }

    println!("Loaded {} tris", triangles.len());
    Ok(TriangleMesh::new(
        vertex_positions,
        vertex_uvs,
        vertex_normals,
        triangles,
        material,
    ))
}
