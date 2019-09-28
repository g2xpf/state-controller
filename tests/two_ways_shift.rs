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

impl Receiver<ThirdState> for InitState {
    type Message = u64;

    fn receive(&mut self, message: Self::Message) {
        println!("### message received from ThirdState: {:?} ###", message);
        self.counter = message;
    }
}

impl EventHandler for InitState {}

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

        match self.counter {
            10 => self.shift_with::<SecondState>(shifter, self.counter),
            30 => self.shift_with::<ThirdState>(shifter, self.counter),
            50 => std::process::exit(0),
            _ => (),
        }
    }
}

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

        if self.counter == 20 {
            self.shift_with::<InitState>(shifter, self.counter);
        }
    }
}

impl EventHandler for SecondState {}

impl State for SecondState {}

#[derive(Default)]
pub struct ThirdState {
    counter: u64,
}

impl Receiver<InitState> for ThirdState {
    type Message = u64;

    fn receive(&mut self, message: Self::Message) {
        println!("### message received from InitState: {:?} ###", message);
        self.counter = message;
    }
}

impl Renderable for ThirdState {
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {
        println!(
            "ThirdState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for ThirdState {
    fn update(&mut self, shifter: &mut Shifter) {
        self.counter += 1;

        if self.counter == 40 {
            self.shift_with::<InitState>(shifter, self.counter);
        }
    }
}

impl EventHandler for ThirdState {}
impl State for ThirdState {}

#[test]
fn two_ways_shift() {
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
    let third_state: ThirdState = Default::default();
    let mut world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .register_state(third_state)
        .finalize();

    world.run();
}
