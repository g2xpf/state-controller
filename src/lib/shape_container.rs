use crate::{renderer::RenderContext, shapes::Shape};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct TextContainer;

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
    <S as Shape>::Vertex: glium::Vertex,
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

    pub fn push(&mut self, s: S) {
        self.shapes.push(s);
    }

    pub fn clear(&mut self) {
        self.shapes = Vec::new();
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
