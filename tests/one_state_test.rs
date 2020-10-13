extern crate state_controller;

use glium::glutin::{event_loop, window};
use glium::Frame;
use state_controller::{EventHandler, Renderable, Shifter, State, Updatable, World};

#[derive(Default)]
pub struct InitState {
    counter: u64,
}

impl Renderable for InitState {
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for InitState {
    fn update(&mut self, _shifter: &mut Shifter) {
        self.counter += 1;
        if self.counter >= 10 {
            std::process::exit(0);
        }
    }
}

impl EventHandler for InitState {}
impl State for InitState {}

#[test]
fn one_state() {
    std::thread::sleep(std::time::Duration::from_millis(1000));

    use glium::glutin;
    let events_loop = event_loop::EventLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(640f64, 640f64);
    let window = window::WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("Main");
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let init_state: InitState = Default::default();
    let world = World::new(events_loop, display, init_state).finalize();
    world.run();
}
