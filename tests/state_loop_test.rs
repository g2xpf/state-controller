use state_controller::{
    ControllerMode::Running, Receiver, Renderable, StateController, Updatable, World,
};

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
    fn render(&self) {
        println!(
            "InitState is rendering...\ncurrent count is: {}",
            self.counter
        );
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

impl Updatable for InitState {
    fn update(&mut self, state_controller: &mut StateController<Running>) {
        self.counter += 1;

        if self.counter % 10 == 0 {
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
        println!("### message received from InitState: {:?} ###", message);
        self.counter = message;
    }
}

impl Renderable for SecondState {
    fn render(&self) {
        println!(
            "SecondState is rendering...\ncurrent count is: {}",
            self.counter
        );
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

impl Updatable for SecondState {
    fn update(&mut self, state_controller: &mut StateController<Running>) {
        self.counter += 1;

        match self.counter % 20 {
            0 if self.counter == 40 => std::process::exit(0),
            0 => state_controller.shift::<Self, InitState>(self.counter),
            _ => (),
        }
    }
}

#[test]
fn state_loop_test() {
    let init_state: InitState = Default::default();
    let second_state: SecondState = Default::default();
    let mut world = World::new(init_state);
    world.register(second_state);

    let mut world = world.finalize();
    world.run();
}