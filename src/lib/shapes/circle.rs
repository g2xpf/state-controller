use super::Shape;
use crate::render_context::RenderContext;
use glium::{
    index::{IndexBuffer, PrimitiveType},
    Display, Program, VertexBuffer,
};

const VSRC: &'static str = include_str!("circle.vert");
const FSRC: &'static str = include_str!("circle.frag");

const VERTICES: &'_ [Vertex; 4] = &[
    Vertex { coord: [-1., -1.] },
    Vertex { coord: [1., -1.] },
    Vertex { coord: [1., 1.] },
    Vertex { coord: [-1., 1.] },
];

const INDICES: &'_ [u32; 4] = &[0u32, 1, 2, 3];

pub struct Circle {
    pub pos: [f32; 2],
    pub r: f32,
    pub color: [f32; 3],
}

pub struct CircleContext {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
}

impl CircleContext {
    pub fn new(display: &Display) -> Self {
        let vertex_buffer = VertexBuffer::new(display, VERTICES).unwrap();
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TriangleFan, INDICES).unwrap();
        CircleContext {
            program: Program::from_source(display, VSRC, FSRC, None).unwrap(),
            vertex_buffer,
            index_buffer,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    coord: [f32; 2],
}

implement_vertex!(Vertex, coord);

impl<'a> Shape<'a> for Circle {
    type Vertex = &'a VertexBuffer<Vertex>;
    type Index = &'a IndexBuffer<u32>;
}

impl<'a> RenderContext<'a> for CircleContext {
    type Target = Circle;

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

implement_render!(CircleContext; pos, r, color; camera_pos: (f32, f32), iResolution: (i32, i32));
