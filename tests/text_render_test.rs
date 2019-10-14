use glium::{glutin, Frame};
use state_controller::{
    primitive_shape::Text,
    utils::{Font, FontStyler},
    Event, EventHandler, Key, PolyShapeContainer, Renderable, Shifter, State, Updatable, World,
};

struct InitState {
    text_container: PolyShapeContainer<Text>,
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

        font.layout_paragraph();
    }
}

impl Updatable for InitState {
    fn update(&mut self, _shifter: &mut Shifter) {
        println!("{:?}", self.text_container[0].font.text);
    }
}

impl State for InitState {}

#[test]
fn text_render_test() {
    let window_dim = (640., 640.).into();
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions(window_dim);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let font_styler = FontStyler::new(
        &display,
        Font::new(include_bytes!("../static/GenRyuMinJP-Regular.ttf")),
        window_dim,
    );

    let init_state = InitState {
        text_container: {
            let mut pc = PolyShapeContainer::new(&display);
            pc.push(Text {
                font: font_styler,
                pos: [0., 0.],
                theta: 0.,
            });
            pc
        },
    };

    let mut world = World::new(events_loop, display, init_state).finalize();
    world.run();
}
