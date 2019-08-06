use super::Shape;
use glium::{index::PrimitiveType, texture};
use std::ops::Deref;
use std::rc::Rc;

const VSRC: &'static str = include_str!("texture.vert");
const FSRC: &'static str = include_str!("texture.frag");

#[derive(Debug, Clone)]
pub struct Texture {
    pub pos: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub tex: Rc<texture::Texture2d>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    coord: [f32; 2],
}

implement_vertex!(Vertex, coord);

impl Shape for Texture {
    type Vertex = Vertex;

    fn vertex() -> Vec<Self::Vertex> {
        vec![
            Vertex {
                coord: [-1.0, -1.0],
            },
            Vertex { coord: [-1.0, 1.0] },
            Vertex { coord: [1.0, 1.0] },
            Vertex { coord: [1.0, -1.0] },
        ]
    }

    fn index() -> Vec<u32> {
        (0..=3).collect()
    }

    fn render_mode() -> PrimitiveType {
        PrimitiveType::TriangleFan
    }

    fn vertex_src() -> &'static str {
        VSRC
    }

    fn fragment_src() -> &'static str {
        FSRC
    }
}

impl_shape_container!(Texture; |shape| {
    pos: shape.pos,
    width: shape.width,
    height: shape.height,
    angle: shape.angle,
    tex: shape.tex.deref()
});
