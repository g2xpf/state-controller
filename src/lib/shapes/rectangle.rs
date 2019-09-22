use super::Shape;
const VSRC: &'static str = include_str!("rectangle.vert");
const FSRC: &'static str = include_str!("rectangle.frag");

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos: [f32; 2],
    pub width: f32,
    pub height: f32,
    pub color: [f32; 3],
    pub angle: f32,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    coord: [f32; 2],
}

implement_vertex!(Vertex, coord);

impl Shape for Rectangle {
    type Vertex = Vertex;

    fn vertex() -> Vec<Self::Vertex> {
        vec![
            Vertex {
                coord: [-1.0, -1.0],
            },
            Vertex { coord: [-1.0, 1.0] },
            Vertex { coord: [1.0, -1.0] },
            Vertex { coord: [1.0, 1.0] },
        ]
    }

    fn index() -> Vec<u32> {
        vec![0, 1, 2, 1, 2, 3]
    }

    fn vertex_src() -> &'static str {
        VSRC
    }

    fn fragment_src() -> &'static str {
        FSRC
    }
}

impl_shape_container!(Rectangle; pos, width, height, color, angle; camera_pos: (f32, f32));
