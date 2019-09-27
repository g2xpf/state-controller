use crate::renderer::RenderContext;
use glium::{index, Vertex};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

mod circle;
mod rectangle;
mod texture;

pub mod primitive_shape {
    pub use super::circle::Circle;
    pub use super::rectangle::Rectangle;
    pub use super::texture::Texture;
}

pub trait Shape {
    type Vertex: Vertex;

    fn vertex() -> Vec<Self::Vertex>;
    fn index() -> Vec<u32>;
    fn render_mode() -> index::PrimitiveType {
        index::PrimitiveType::TrianglesList
    }

    fn vertex_src() -> &'static str;
    fn fragment_src() -> &'static str;
}

pub struct ShapeContainer<S>
where
    S: Shape,
{
    pub shapes: Vec<S>,
    pub render_context: Rc<RenderContext<S>>,
}

impl<S> ShapeContainer<S>
where
    S: Shape,
{
    pub fn new<'a>(display: &'a glium::Display) -> Self {
        ShapeContainer {
            shapes: Vec::new(),
            render_context: Rc::new(RenderContext::<S>::new(display)),
        }
    }

    pub fn clone_context<'a>(&self) -> Self {
        ShapeContainer {
            shapes: Vec::new(),
            render_context: Rc::clone(&self.render_context),
        }
    }
}

impl<S> Deref for ShapeContainer<S>
where
    S: Shape,
{
    type Target = Vec<S>;
    fn deref(&self) -> &Self::Target {
        &self.shapes
    }
}

impl<S> DerefMut for ShapeContainer<S>
where
    S: Shape,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shapes
    }
}
