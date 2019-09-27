use crate::renderer::RawRenderContext;
use glium::{index, Display, Vertex};
use std::ops::{Deref, DerefMut};

mod text;

pub use text::Text;

pub trait PolyShape {
    type Vertex: Vertex;

    fn vertex_index(&self) -> (Vec<Self::Vertex>, Vec<u32>);
    fn render_mode() -> index::PrimitiveType {
        index::PrimitiveType::TrianglesList
    }

    fn vertex_src() -> &'static str;
    fn fragment_src() -> &'static str;
}

pub struct PolyShapeContainer<S> {
    pub shapes: Vec<S>,
    pub raw_render_context: RawRenderContext,
}

impl<S> PolyShapeContainer<S>
where
    S: PolyShape,
{
    pub fn new(display: &Display) -> Self {
        PolyShapeContainer {
            shapes: Vec::new(),
            raw_render_context: RawRenderContext::new::<S>(display),
        }
    }

    pub fn clone_conext(&self) -> Self {
        PolyShapeContainer {
            shapes: Vec::new(),
            raw_render_context: self.raw_render_context.clone(),
        }
    }
}

impl<S> Deref for PolyShapeContainer<S>
where
    S: PolyShape,
{
    type Target = Vec<S>;
    fn deref(&self) -> &Self::Target {
        &self.shapes
    }
}

impl<S> DerefMut for PolyShapeContainer<S>
where
    S: PolyShape,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shapes
    }
}
