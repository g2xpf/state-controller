use super::PolyShape;
use crate::utils::FontStyler;
use glium::index;
use glium::uniforms::MagnifySamplerFilter;

pub struct Text {
    pub font: FontStyler<'static>,
    pub pos: [f32; 2],
    pub theta: f32,
}

const VSRC: &str = include_str!("text.vert");
const FSRC: &str = include_str!("text.frag");

#[derive(Copy, Clone)]
pub struct Vertex {
    pub coord: [f32; 2],
    pub tex_coord: [f32; 2],
}

implement_vertex!(Vertex, coord, tex_coord);

impl PolyShape for Text {
    type Vertex = Vertex;

    fn vertex_index(&self) -> (Vec<Self::Vertex>, Vec<u32>) {
        let glyphs: Vec<_> = self
            .font
            .glyphs
            .iter()
            .flat_map(|g| {
                if let Some((uv, position)) = self.font.get_glyph_info(g) {
                    vec![
                        Vertex {
                            coord: position.left_bottom,
                            tex_coord: uv.left_bottom,
                        },
                        Vertex {
                            coord: position.right_bottom,
                            tex_coord: uv.right_bottom,
                        },
                        Vertex {
                            coord: position.right_top,
                            tex_coord: uv.right_top,
                        },
                        Vertex {
                            coord: position.right_top,
                            tex_coord: uv.right_top,
                        },
                        Vertex {
                            coord: position.left_top,
                            tex_coord: uv.left_top,
                        },
                        Vertex {
                            coord: position.left_bottom,
                            tex_coord: uv.left_bottom,
                        },
                    ]
                } else {
                    vec![]
                }
            })
            .collect();
        (glyphs, vec![])
    }

    fn render_mode() -> index::PrimitiveType {
        index::PrimitiveType::TrianglesList
    }

    fn vertex_src() -> &'static str {
        VSRC
    }

    fn fragment_src() -> &'static str {
        FSRC
    }
}

impl_polyshape_container!(Text; |text| {
    pos: text.pos,
    theta: text.theta,
    tex: text.font.get_texture(MagnifySamplerFilter::Nearest)
}; color: (f32, f32, f32), camera_pos: (f32, f32));
