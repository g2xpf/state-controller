extern crate state_controller;

use state_controller::{
    glium::glutin, Event, EventHandler, Key, Parent, Renderable, Shifter, State, Updatable, World,
};

struct InitState;

impl Renderable for InitState {}
impl Updatable for InitState {}
impl EventHandler for InitState {
    fn handle(&mut self, shifter: &Shifter, event: &Event) {
        self.parent_handle::<Global>(shifter, event);
    }
}
impl State for InitState {}

struct Global;

impl EventHandler for Global {
    fn handle_by_ref(&self, event: &Event) {
        if event.key(Key::Escape).is_pressed() {
            std::process::exit(0);
        }
    }
}

impl Updatable for Global {}
impl Renderable for Global {}
impl State for Global {}

impl Parent<InitState> for Global {}

#[test]
fn call_parent_handle() {
    let events_loop = glutin::EventsLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(640f64, 640f64);
    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("Main");
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut world = World::new(events_loop, display, InitState)
        .register_state(Global)
        .finalize();

    world.run();
}
