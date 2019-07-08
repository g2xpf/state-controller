extern crate state_controller;

use glium::{Frame, Surface};
use state_controller::{
    utils::{Linear, Timer},
    EventHandler, Renderable, Shifter, State, Updatable, World,
};
#[derive(Default)]

pub struct InitState {
    counter: u64,
    timer: Timer,
}

impl Renderable for InitState {
    fn render(&self, frame: &mut Frame) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
        frame.clear_color(0.5, 0.5, 0.5, 1.0);
        frame.set_finish().unwrap();
    }
}

impl Updatable for InitState {
    fn update(&mut self, _shifter: &mut Shifter) {
        if let Some(ratio) = self.timer.get_ratio_easing::<Linear>() {
            println!("progress: {}", ratio);
        } else {
            std::process::exit(0);
        }
    }
}

impl EventHandler for InitState {}
impl State for InitState {
    fn initialize(&mut self) {
        self.timer = Timer::from_secs(3);
        self.timer.start();
    }
}

#[test]
fn one_state() {
    std::thread::sleep(std::time::Duration::from_millis(1000));

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
