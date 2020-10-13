// state family: InitState -> SecondState
use glium::glutin::{event_loop, window};
use state_controller::{EventHandler, Parent, Renderable, Shifter, State, Updatable, World};

#[derive(Default)]
struct InitState {
    counter: i32,
}

impl Renderable for InitState {}

impl Updatable for InitState {
    fn update(&mut self, shifter: &mut Shifter) {
        let mut parent = self.parent_mut::<SecondState>(shifter);
        assert_eq!(self.counter, parent.counter);

        parent.counter += 1;
        self.counter += 1;

        if self.counter >= 10 {
            std::process::exit(0);
        }
    }
}

impl EventHandler for InitState {}
impl State for InitState {}

#[derive(Default)]
struct SecondState {
    counter: i32,
}

impl Renderable for SecondState {}
impl Updatable for SecondState {}
impl EventHandler for SecondState {}
impl State for SecondState {}

impl Parent<InitState> for SecondState {}

#[test]
fn parent_access() {
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
    let second_state: SecondState = Default::default();
    let world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .finalize();

    world.run();
}
