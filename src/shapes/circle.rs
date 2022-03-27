use super::Shape;
use glium::index::PrimitiveType;

const VSRC: &'static str = include_str!("circle.vert");
const FSRC: &'static str = include_str!("circle.frag");

pub struct Circle {
    pub pos: [f32; 2],
    pub r: f32,
    pub color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Vertex {
    coord: [f32; 2],
}

implement_vertex!(Vertex, coord);

impl Shape for Circle {
    type Vertex = Vertex;

    fn vertex() -> Vec<Self::Vertex> {
        vec![
            Vertex {
                coord: [-1.0, -1.0],
            },
            Vertex { coord: [1.0, -1.0] },
            Vertex { coord: [1.0, 1.0] },
            Vertex { coord: [-1.0, 1.0] },
        ]
    }

    fn index() -> Vec<u32> {
        (0..4).collect()
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

impl_shape_container!(Circle; pos, r, color; camera_pos: (f32, f32), iResolution: (i32, i32));
