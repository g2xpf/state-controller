use glium::{Frame, Surface};
use state_controller::{EventHandler, Receiver, Renderable, Shifter, State, Updatable, World};

#[derive(Default)]
pub struct InitState {
    counter: u64,
}

impl Renderable for InitState {
    fn render(&self, frame: &mut Frame) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );

        frame.clear_color(0.2, 0.2, 0.2, 1.0);
        frame.set_finish().unwrap();
    }
}

impl Updatable for InitState {
    fn update(&mut self, state_shifter: &mut Shifter) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 10 {
            self.shift_with::<SecondState>(state_shifter, self.counter);
        }
    }
}

impl State for InitState {}

impl EventHandler for InitState {}

#[derive(Default)]
pub struct SecondState {
    counter: u64,
}

impl Receiver<InitState> for SecondState {
    type Message = u64;

    fn receive(&mut self, message: Self::Message) {
        println!("### message received: {:?} ###", message);
        self.counter = message;
    }
}

impl Renderable for SecondState {
    fn render(&self, frame: &mut Frame) {
        println!(
            "SecondState is rendering...\ncurrent count is: {}",
            self.counter
        );

        frame.clear_color(0.2, 0.0, 0.0, 1.0);
        frame.set_finish().unwrap();
    }
}

impl Updatable for SecondState {
    fn update(&mut self, _shifter: &mut Shifter) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 30 {
            std::process::exit(0);
        }
    }
}

impl State for SecondState {}

impl EventHandler for SecondState {}

#[test]
fn two_states_test() {
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
    let second_state: SecondState = Default::default();
    let mut world = World::new(events_loop, display, init_state);
    world.register(second_state);

    let mut world = world.finalize();
    world.run();
}
