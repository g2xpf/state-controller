use crate::shapes::Shape;
use glium::{self, Display, IndexBuffer, Program, VertexBuffer};

pub struct RenderContext<S>
where
    S: Shape,
{
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<S::Vertex>,
    pub index_buffer: glium::IndexBuffer<u32>,
}

impl<S> RenderContext<S>
where
    S: Shape,
{
    pub(crate) fn new(display: &Display) -> Self {
        RenderContext {
            program: Program::from_source(display, S::vertex_src(), S::fragment_src(), None)
                .unwrap(),
            vertex_buffer: VertexBuffer::new(display, &S::vertex()).unwrap(),
            index_buffer: IndexBuffer::new(display, S::render_mode(), &S::index()).unwrap(),
        }
    }
}
