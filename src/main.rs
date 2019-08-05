extern crate state_controller;

use glium::{glutin, Frame, Surface};
use state_controller::{
    primitive_shape::{Circle, Rectangle},
    Event, EventHandler, Key, Receiver, Renderable, ShapeContainer, Shifter, State, Updatable,
    World,
};

struct InitState {
    counter: u64,
    rectangle_container: ShapeContainer<Rectangle>,
}

impl Renderable for InitState {
    fn render(&self, frame: &mut Frame) {
        frame.clear_color(0.1, 0.1, 0.1, 1.0);
        self.rectangle_container.render(frame, &Default::default());
        frame.set_finish().unwrap();
    }
}

impl Updatable for InitState {
    fn update(&mut self, shifter: &mut Shifter) {
        if self.counter >= 1000 {
            self.counter = 0;
            shifter.shift::<Self, SecondState>(());
        }
        self.counter += 1;
    }
}

impl EventHandler for InitState {
    fn handle(&mut self, event: &Event) {
        if event.window.close_requested || event.key(Key::Escape).is_pressed() {
            std::process::exit(0)
        }
        for rectangle in self.rectangle_container.iter_mut() {
            if event.key(Key::D).is_pressed() {
                rectangle.pos[0] += 0.03;
                println!("Right is pressed!");
            }
            if event.key(Key::A).is_pressed() {
                rectangle.pos[0] -= 0.03;
            }
            if event.key(Key::W).is_pressed() {
                rectangle.pos[1] += 0.03;
            }
            if event.key(Key::S).is_pressed() {
                rectangle.pos[1] -= 0.03;
            }
            if event.key(Key::L).is_pressed() {
                rectangle.angle -= 0.03;
            }
            if event.key(Key::H).is_pressed() {
                rectangle.angle += 0.03;
            }
            if event.key(Key::K).is_pressed() {
                self.counter = 10000;
            }
        }
    }
}

impl State for InitState {}

struct SecondState {
    counter: u64,
    circle_container: ShapeContainer<Circle>,
}

impl Renderable for SecondState {
    fn render(&self, frame: &mut Frame) {
        frame.clear_color(0.1, 0.2, 0.2, 1.0);
        self.circle_container.render(frame, &Default::default());
        frame.set_finish().unwrap();
    }
}

impl Updatable for SecondState {
    fn update(&mut self, shifter: &mut Shifter) {
        if self.counter >= 1000 {
            self.counter = 0;
            shifter.shift::<Self, InitState>(());
        }
        self.counter += 1;
    }
}

impl EventHandler for SecondState {
    fn handle(&mut self, event: &Event) {
        if event.window.close_requested || event.key(Key::Escape).is_pressed() {
            std::process::exit(0)
        }
        for circle in self.circle_container.iter_mut() {
            if event.key(Key::D).is_pressed() {
                circle.pos[0] += 0.03;
                println!("Right is pressed!");
            }
            if event.key(Key::A).is_pressed() {
                circle.pos[0] -= 0.03;
            }
            if event.key(Key::W).is_pressed() {
                circle.pos[1] += 0.03;
            }
            if event.key(Key::S).is_pressed() {
                circle.pos[1] -= 0.03;
            }
            if event.key(Key::J).is_pressed() {
                self.counter = 10000;
            }
        }
    }
}

impl State for SecondState {}

impl Receiver<SecondState> for InitState {
    type Message = ();

    fn receive(&mut self, _message: Self::Message) {}
}

impl Receiver<InitState> for SecondState {
    type Message = ();

    fn receive(&mut self, _message: Self::Message) {}
}

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(640f64, 640f64);
    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("GLWindow");
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut rectangle_container = ShapeContainer::<Rectangle>::new(&display);
    rectangle_container.push(Rectangle {
        pos: [0.0, 0.0],
        width: 0.2,
        height: 0.5,
        color: [0.1, 0.2, 0.1],
        angle: std::f32::consts::PI / 3.0,
    });

    let init_state: InitState = InitState {
        counter: 0,
        rectangle_container,
    };

    let mut circle_container = ShapeContainer::<Circle>::new(&display);
    circle_container.push(Circle {
        pos: [0.0, 0.0],
        r: 1.3,
        color: [0.0, 0.4, 0.4],
    });

    let second_state: SecondState = SecondState {
        counter: 0,
        circle_container,
    };

    let mut world = World::new(events_loop, display, init_state);
    world.register(second_state);
    let mut world = world.finalize();
    world.run();
}
