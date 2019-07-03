extern crate state_controller;

use glium::{
    glutin::{self, Event},
    Frame, Surface,
};
use state_controller::{EventHandler, Renderable, Shifter, State, Updatable, World};

#[derive(Default)]
struct InitState {
    counter: u64,
}

impl Renderable for InitState {
    fn render(&self, frame: &mut Frame) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
        frame.clear_color(1.0, 0.0, 0.0, 1.0);
        frame.set_finish().unwrap();
    }
}

impl Updatable for InitState {
    fn update(&mut self, _shifter: &mut Shifter) {
        self.counter += 1;
    }
}

impl EventHandler for InitState {
    fn handle(&mut self, event: &Event) {
        match event {
            glutin::Event::WindowEvent {
                event: glutin::WindowEvent::CloseRequested,
                ..
            }
            | glutin::Event::WindowEvent {
                event:
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                state: glutin::ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => std::process::exit(0),
            _ => (),
        }
        println!("{:?}\n", event);
    }
}

impl State for InitState {}

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(640f64, 640f64);
    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("Main");
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let init_state: InitState = Default::default();
    let mut world = World::new(events_loop, display, init_state).finalize();
    world.run();
}
