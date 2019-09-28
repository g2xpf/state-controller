#[macro_use]
extern crate glium;

use glium::{
    glutin, index, uniforms, Blend, DrawParameters, Frame, Program, Surface, VertexBuffer,
};
use state_controller::{
    primitive_shape::Text, utils::FontStyler, Event, EventHandler, Key, PolyShapeContainer,
    Receiver, Renderable, Shifter, State, Updatable, World,
};

struct InitState {
    text_container: PolyShapeContainer<Text>,
    shift: bool,
}

impl Renderable for InitState {
    fn render(&self, _shifter: &Shifter, frame: &mut Frame) {
        self.text_container
            .render(frame, &Default::default(), (1., 1., 1.), (0., 0.));
    }
}

impl EventHandler for InitState {
    fn handle(&mut self, event: &Event) {
        let font = &mut self.text_container[0].font;
        for c in event.text().chars() {
            if c as u8 == 0x8 {
                font.text.pop();
            } else {
                font.text.push(c);
            }
        }

        if event.key(Key::Escape).is_pressed() {
            std::process::exit(0);
        }

        if event.key(Key::Right).is_pressed() {
            self.shift = true;
        }

        font.layout_paragraph();
    }
}

impl Updatable for InitState {
    fn update(&mut self, shifter: &mut Shifter) {
        println!("{:?}", self.text_container[0].font.text);
        if self.shift {
            self.shift = false;
            self.shift::<SecondState>(shifter);
        }
    }
}

impl State for InitState {}

impl Receiver<SecondState> for InitState {
    type Message = ();
    fn receive(&mut self, _msg: Self::Message) {}
}

#[derive(Copy, Clone)]
struct Vertex {
    coord: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, coord, tex_coord);

struct SecondState {
    font_styler: FontStyler<'static>,
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    shift: bool,
}

impl Renderable for SecondState {
    fn render(&self, _shifter: &Shifter, frame: &mut Frame) {
        let font_styler = &self.font_styler;
        for glyph in font_styler.glyphs.iter() {
            if let Some((uv, pos)) = font_styler.get_glyph_info(glyph) {
                self.vertex_buffer.write(&[
                    Vertex {
                        coord: pos.left_bottom,
                        tex_coord: uv.left_bottom,
                    },
                    Vertex {
                        coord: pos.right_bottom,
                        tex_coord: uv.right_bottom,
                    },
                    Vertex {
                        coord: pos.right_top,
                        tex_coord: uv.right_top,
                    },
                    Vertex {
                        coord: pos.right_top,
                        tex_coord: uv.right_top,
                    },
                    Vertex {
                        coord: pos.left_top,
                        tex_coord: uv.left_top,
                    },
                    Vertex {
                        coord: pos.left_bottom,
                        tex_coord: uv.left_bottom,
                    },
                ]);
                frame
                    .draw(
                        &self.vertex_buffer,
                        &index::NoIndices(index::PrimitiveType::TrianglesList),
                        &self.program,
                        &uniform! {color: (1f32, 1f32, 1f32), tex: font_styler.get_texture(uniforms::MagnifySamplerFilter::Nearest)},
                        &DrawParameters {
                            blend: Blend::alpha_blending(),
                            ..Default::default()
                        },
                    )
                    .unwrap();
            }
        }
    }
}

impl Updatable for SecondState {
    fn update(&mut self, shifter: &mut Shifter) {
        if self.shift {
            self.shift = false;
            self.shift::<InitState>(shifter);
        }
    }
}

impl EventHandler for SecondState {
    fn handle(&mut self, event: &Event) {
        let font = &mut self.font_styler;
        for c in event.text().chars() {
            if c as u8 == 0x8 {
                font.text.pop();
            } else {
                font.text.push(c);
            }
        }

        if event.key(Key::Escape).is_pressed() {
            std::process::exit(0);
        }

        if event.key(Key::Left).is_pressed() {
            self.shift = true;
        }

        font.layout_paragraph();
    }
}

impl State for SecondState {}

impl Receiver<InitState> for SecondState {
    type Message = ();
    fn receive(&mut self, _msg: Self::Message) {}
}

#[test]
fn text_render_test() {
    let window_dim = (640., 640.).into();
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions(window_dim);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let font = include_bytes!("../static/GenRyuMinJP-Regular.ttf");

    let init_state = InitState {
        shift: false,
        text_container: {
            let mut pc = PolyShapeContainer::new(&display);
            pc.push(Text {
                font: FontStyler::new(&display, font, window_dim),
                pos: [0., 0.],
                theta: 0.,
            });
            pc
        },
    };

    let second_state = SecondState {
        program: Program::from_source(
            &display,
            include_str!("text_render_test.vert"),
            include_str!("text_render_test.frag"),
            None,
        )
        .unwrap(),
        vertex_buffer: VertexBuffer::empty_dynamic(&display, 6).unwrap(),
        shift: false,
        font_styler: FontStyler::new(&display, font, window_dim),
    };

    let mut world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .finalize();
    world.run();
}
