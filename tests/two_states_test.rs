use state_controller::{
    ControllerMode::Running, Receiver, Renderable, StateController, Updatable, World,
};

#[derive(Default)]
pub struct InitState {
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
    fn update(&mut self, state_controller: &mut StateController<Running>) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 10 {
            state_controller.shift::<Self, SecondState>(self.counter);
        }
    }
}

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
    fn render(&self) {
        println!(
            "SecondState is rendering...\ncurrent count is: {}",
            self.counter
        );
    }
}

impl Updatable for SecondState {
    fn update(&mut self, _state_controller: &mut StateController<Running>) {
        self.counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
        if self.counter >= 30 {
            std::process::exit(0);
        }
    }
}

#[test]
fn two_states_test() {
    let init_state: InitState = Default::default();
    let second_state: SecondState = Default::default();
    let mut world = World::new(init_state);
    world.register(second_state);

    let mut world = world.finalize();
    world.run();
}