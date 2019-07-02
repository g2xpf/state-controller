use state_controller::{Receiver, Renderable, Shifter, Updatable, World};

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
    fn update(&mut self, state_controller: &mut Shifter) {
        self.counter += 1;

        match self.counter {
            10 => state_controller.shift::<Self, SecondState>(self.counter),
            30 => state_controller.shift::<Self, ThirdState>(self.counter),
            50 => std::process::exit(0),
            _ => (),
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
    fn update(&mut self, state_controller: &mut Shifter) {
        self.counter += 1;

        if self.counter == 20 {
            state_controller.shift::<Self, InitState>(self.counter);
        }
    }
}

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
    fn render(&self) {
        println!(
            "ThirdState is rendering...\ncurrent count is: {}",
            self.counter
        );
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

impl Updatable for ThirdState {
    fn update(&mut self, state_controller: &mut Shifter) {
        self.counter += 1;

        if self.counter == 40 {
            state_controller.shift::<Self, InitState>(self.counter);
        }
    }
}

#[test]
fn two_ways_shift() {
    let init_state: InitState = Default::default();
    let second_state: SecondState = Default::default();
    let third_state: ThirdState = Default::default();
    let mut world = World::new(init_state);
    world.register(second_state);
    world.register(third_state);

    let mut world = world.finalize();
    world.run();
}
