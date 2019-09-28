extern crate state_controller;

use glium::Frame;
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
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for InitState {
    fn update(&mut self, _shifter: &mut Shifter) {
        match self.timer.is_over() {
            Some(true) => std::process::exit(0),
            _ => match self.timer.get_ratio_easing::<Linear>() {
                Some(ratio) => println!("progress: {}", ratio),
                _ => {}
            },
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
