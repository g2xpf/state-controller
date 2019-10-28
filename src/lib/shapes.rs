use glium::{index::IndicesSource, vertex::VerticesSource};

mod circle;
mod rectangle;
mod texture;

pub use circle::{Circle, CircleContext};
pub use rectangle::{Rectangle, RectangleContext};
// pub use rectangle::Rectangle;
pub use texture::{Texture, TextureContext};

pub trait Shape<'a> {
    type Vertex: Into<VerticesSource<'a>>;
    type Index: Into<IndicesSource<'a>>;
}
