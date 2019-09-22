use super::Shape;
use glium::index::PrimitiveType;

const VSRC: &'static str = include_str!("circle.vert");
const FSRC: &'static str = include_str!("circle.frag");
const NSEP: u32 = 50;

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
        let mut v = vec![Vertex { coord: [0.0, 0.0] }];
        for i in 0..NSEP {
            let theta = (i as f32) / (NSEP as f32) * 2.0 * std::f32::consts::PI;
            v.push(Vertex {
                coord: [0.5 * theta.cos(), 0.5 * theta.sin()],
            });
        }
        v
    }

    fn index() -> Vec<u32> {
        (0u32..NSEP).chain(Some(1).into_iter()).collect()
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

impl_shape_container!(Circle; pos, r, color; camera_pos: (f32, f32));
