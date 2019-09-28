use glium::Frame;
use state_controller::{EventHandler, Receiver, Renderable, Shifter, State, Updatable, World};

#[derive(Default)]
pub struct InitState {
    counter: u64,
}

impl Receiver<SecondState> for InitState {
    type Message = u64;

    fn receive(&mut self, message: Self::Message) {
        println!("### message received from SecondState: {:?} ###", message);
        self.counter = message;
    }
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
    fn update(&mut self, shifter: &mut Shifter) {
        self.counter += 1;

        if self.counter % 10 == 0 {
            self.shift_with::<SecondState>(shifter, self.counter);
        }
    }
}

impl EventHandler for InitState {}
impl State for InitState {}

#[derive(Default)]
pub struct SecondState {
    counter: u64,
}

impl Receiver<InitState> for SecondState {
    type Message = u64;

    fn receive(&mut self, message: Self::Message) {
        println!("### message received from InitState: {:?} ###", message);
        self.counter = message;
    }
}

impl Renderable for SecondState {
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {
        println!(
            "SecondState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for SecondState {
    fn update(&mut self, shifter: &mut Shifter) {
        self.counter += 1;

        match self.counter % 20 {
            0 if self.counter == 40 => std::process::exit(0),
            0 => self.shift_with::<InitState>(shifter, self.counter),
            _ => (),
        }
    }
}

impl EventHandler for SecondState {}
impl State for SecondState {}

#[test]
fn state_loop_test() {
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
    let mut world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .finalize();

    world.run();
}
