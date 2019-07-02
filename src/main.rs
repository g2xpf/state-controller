extern crate state_controller;

use glium::{glutin::Event, Frame, Surface};
use state_controller::{Renderable, Shifter, Updatable, World};

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
    fn update(&mut self, _state_controller: &mut Shifter, events: &Vec<Event>) {
        self.counter += 1;
        for event in events.iter() {
            println!("{:?}\n", event);
        }
    }
}

fn main() {
    use glium::glutin;
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
