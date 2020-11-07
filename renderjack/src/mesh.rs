use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};

use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub tangent: Vector3<f32>,
}

impl Vertex {
    pub fn calculate_tangent(x: Vertex, y: Vertex, z: Vertex) -> Vector3<f32> {
        let edge1 = y.position - x.position;
        let edge2 = z.position - x.position;

        let uv1 = y.uv - x.uv;
        let uv2 = z.uv - x.uv;

        let f = 1.0 / (uv1.x * uv2.y - uv2.x * uv1.y);
        let mut tangent = Vector3::new(0.0, 0.0, 0.0);
        tangent.x = f * (uv2.y * edge1.x - uv1.y * edge2.x);
        tangent.y = f * (uv2.y * edge1.y - uv1.y * edge2.y);
        tangent.z = f * (uv2.y * edge1.z - uv1.y * edge2.z);

        tangent.normalize()
    }
}

#[derive(Debug)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

#[derive(Debug)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Mesh { triangles }
    }
}

pub fn load_ply(path: PathBuf) -> Mesh {
    let buffer = fs::read_to_string(path).unwrap();

    let mut vertices = Vec::<Vertex>::new();
    let mut faces = Vec::<Triangle>::new();
    let mut lines = buffer.lines();

    // Parse header
    'header: while let Some(line) = lines.next() {
        let mut tokens = line.split_whitespace();

        match tokens.next().unwrap() {
            "element" => match tokens.next().unwrap() {
                "vertex" => {
                    vertices.reserve_exact(tokens.next().unwrap().parse::<usize>().unwrap())
                }
                _ => {}
            },
            "end_header" => break 'header,
            _ => {}
        }
    }

    // Parse vertices
    for _ in 0..vertices.capacity() {
        let line = lines.next().unwrap();
        let tokens = line.split_whitespace();
        let values = tokens
            .map(|t| t.parse::<f32>().unwrap())
            .collect::<Vec<_>>();
        vertices.push(Vertex {
            position: Vector3::new(values[0], values[1], values[2]),
            normal: Vector3::new(values[3], values[4], values[5]),
            uv: Vector2::new(
                *values.get(6).unwrap_or(&0.0),
                *values.get(7).unwrap_or(&0.0),
            ),
            tangent: Vector3::new(0.0, 0.0, 0.0),
        });
    }

    // Parse faces
    while let Some(line) = lines.next() {
        let tokens = line.split_whitespace();
        let values = tokens
            .map(|t| t.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        match values[0] {
            3 => {
                let face = [values[1], values[2], values[3]];

                let tangent = Vertex::calculate_tangent(
                    vertices[face[0] as usize].clone(),
                    vertices[face[1] as usize].clone(),
                    vertices[face[2] as usize].clone(),
                );
                vertices[face[0] as usize].tangent = tangent;
                vertices[face[1] as usize].tangent = tangent;
                vertices[face[2] as usize].tangent = tangent;

                faces.push(Triangle(
                    vertices[values[1] as usize].clone(),
                    vertices[values[2] as usize].clone(),
                    vertices[values[3] as usize].clone(),
                ));
            }
            _ => {}
        }
    }

    Mesh::new(faces)
}
