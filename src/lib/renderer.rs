use crate::polyshapes::PolyShape;
use crate::shapes::Shape;
use glium::{self, Display, IndexBuffer, Program, VertexBuffer};
use std::error::Error;
use std::rc::Rc;

pub struct RenderContext<S>
where
    S: Shape,
{
    pub program: Program,
    pub vertex_buffer: VertexBuffer<S::Vertex>,
    pub index_buffer: IndexBuffer<u32>,
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

pub struct RawRenderContext {
    pub program: Rc<Program>,
    display: Display,
}

impl RawRenderContext {
    pub(crate) fn new<S>(display: &Display) -> Self
    where
        S: PolyShape,
    {
        RawRenderContext {
            program: Rc::new(
                Program::from_source(display, S::vertex_src(), S::fragment_src(), None).unwrap(),
            ),
            display: display.clone(),
        }
    }

    pub fn create_buffers<S>(
        &self,
        shapes: &Vec<S>,
    ) -> Result<(VertexBuffer<S::Vertex>, IndexBuffer<u32>), Box<dyn Error>>
    where
        S: Shape,
    {
        // TODO: Buffering for faster construction
        let vertex_count = S::vertex().len() as u32;
        let shape_count = shapes.len();
        let vertices: Vec<S::Vertex> = shapes.iter().flat_map(|_| S::vertex()).collect();
        let indices: Vec<u32> = (0..shape_count)
            .flat_map(|i| {
                S::index()
                    .into_iter()
                    .map(move |index| i as u32 + index * vertex_count)
            })
            .collect();
        let vbo = VertexBuffer::new(&self.display, &vertices)?;
        let ibo = IndexBuffer::new(&self.display, S::render_mode(), &indices)?;
        Ok((vbo, ibo))
    }
}

impl Clone for RawRenderContext {
    fn clone(&self) -> Self {
        RawRenderContext {
            program: Rc::clone(&self.program),
            display: self.display.clone(),
        }
    }
}
