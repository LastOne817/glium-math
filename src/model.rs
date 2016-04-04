extern crate glium;
extern crate obj;

use obj::*;
use super::vector::vector3::*;
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub struct ModelVertex {
    pub position: Vector3,
    pub normal: Vector3,
}

implement_vertex!(ModelVertex, position, normal);

pub fn load_object(display: &<glium::glutin::WindowBuilder<'static> as glium::DisplayBuild>::Facade, path: &str) -> (glium::VertexBuffer<ModelVertex>, glium::IndexBuffer<u16>) {
    let obj: Obj = load_obj(BufReader::new(File::open(path).unwrap())).unwrap();
    let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &obj.indices).unwrap();
    let buffer = glium::VertexBuffer::new(display, &obj_to_vertex(obj.vertices.clone())).unwrap();
    return (buffer, indices);
}

fn obj_to_vertex(obj: Vec<Vertex>) -> Vec<ModelVertex> {
    let mut result = Vec::new();
    for v in obj {
        result.push(ModelVertex { position: Vector3::new(v.position[0], v.position[1], v.position[2]),
        normal: Vector3::new(v.normal[0], v.normal[1], v.normal[2])});
    }
    return result;
}
