use crate::Shape;
use glium::{
    index::{IndicesSource, PrimitiveType},
    uniforms::Uniforms,
    DrawParameters, IndexBuffer, Program, VertexBuffer,
};

pub trait RenderContext<'a> {
    type Target: Shape<'a>;

    fn vertex<'b: 'a, 'c: 'a>(&'b self, _: &'c Self::Target)
        -> <Self::Target as Shape<'a>>::Vertex;
    fn index<'b: 'a, 'c: 'a>(&'b self, _: &'c Self::Target) -> <Self::Target as Shape<'a>>::Index;
    fn program(&self, _: &Self::Target) -> &Program;
}
