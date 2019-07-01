extern crate state_controller;

use state_controller::{
    ControllerMode::Running, Renderable, State, StateController, Updatable, World,
};

#[derive(Default)]
struct InitState {
    counter: u64,
}

impl Renderable for InitState {
    fn render(&self) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for InitState {
    fn update(&mut self, _state_controller: &mut StateController<Running>) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

impl State for InitState {}

fn main() {
    let init_state: InitState = Default::default();
    let mut world = World::new(init_state).finalize();
    world.run();
}