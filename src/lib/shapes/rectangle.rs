use crate::RenderContext;
use glium::{index::PrimitiveType, Display, IndexBuffer, Program, VertexBuffer};

use super::Shape;
const VSRC: &'static str = include_str!("rectangle.vert");
const FSRC: &'static str = include_str!("rectangle.frag");

const VERTICES: &'_ [Vertex] = &[
    Vertex {
        coord: [-1.0, -1.0],
    },
    Vertex { coord: [1.0, -1.0] },
    Vertex { coord: [1.0, 1.0] },
    Vertex { coord: [-1.0, 1.0] },
];

const INDICES: &'_ [u32] = &[0, 1, 2, 3];

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub color: [f32; 3],
    pub angle: f32,
}

pub struct RectangleContext {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
}

impl RectangleContext {
    pub fn new(display: &Display) -> Self {
        RectangleContext {
            program: Program::from_source(display, VSRC, FSRC, None).unwrap(),
            vertex_buffer: VertexBuffer::new(display, VERTICES).unwrap(),
            index_buffer: IndexBuffer::new(display, PrimitiveType::TriangleFan, INDICES).unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    coord: [f32; 2],
}

implement_vertex!(Vertex, coord);

impl<'a> Shape<'a> for Rectangle {
    type Vertex = &'a VertexBuffer<Vertex>;
    type Index = &'a IndexBuffer<u32>;
}

impl<'a> RenderContext<'a> for RectangleContext {
    type Target = Rectangle;

    fn vertex<'b: 'a, 'c: 'a>(
        &'b self,
        _: &'c Self::Target,
    ) -> <Self::Target as Shape<'a>>::Vertex {
        &self.vertex_buffer
    }

    fn index<'b: 'a, 'c: 'a>(&'b self, _: &'c Self::Target) -> <Self::Target as Shape<'a>>::Index {
        &self.index_buffer
    }

    fn program(&self, _: &Self::Target) -> &Program {
        &self.program
    }
}

implement_render!(RectangleContext; pos, width, height, color, angle; camera_pos: (f32, f32));
