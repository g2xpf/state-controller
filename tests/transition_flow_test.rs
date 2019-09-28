use glium::Frame;
use state_controller::{
    utils::Timer, Event, EventHandler, IntermediateState, Receiver, Renderable, Shifter, State,
    Transition, TransitionFlow, Transitionable, Updatable, World,
};

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
    fn update(&mut self, state_shifter: &mut Shifter) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 50 {
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
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {
        println!(
            "SecondState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for SecondState {
    fn update(&mut self, _shifter: &mut Shifter) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 100 {
            std::process::exit(0);
        }
    }
}

impl State for SecondState {}

impl EventHandler for SecondState {}

struct InitToSecond {
    timer: Timer,
    transition: Transition<InitState, SecondState>,
}

impl IntermediateState for InitToSecond {
    fn transition_location(&mut self) -> &mut dyn Transitionable {
        &mut self.transition as &mut dyn Transitionable
    }

    fn initialize(&mut self) {
        self.timer.start();
    }

    fn finalize(&mut self) {
        self.timer.stop();
    }

    fn update(&mut self) -> TransitionFlow {
        match self.timer.is_over() {
            Some(true) => TransitionFlow::Break,
            _ => TransitionFlow::Continue,
        }
    }

    fn handle(&mut self, _event: &Event) -> TransitionFlow {
        TransitionFlow::Break
    }
}

#[test]
fn transition_flow_test() {
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

    let init_to_second = InitToSecond {
        timer: Timer::from_millis(3000),
        transition: Default::default(),
    };

    let mut world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .register_transition::<InitState, SecondState, _>(init_to_second)
        .finalize();

    world.run();
}
