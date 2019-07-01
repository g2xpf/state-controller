use crate::{
    controller_mode::{Pending, Running},
    state::State,
    state_controller::StateController,
    types::{StateEntry, StateID},
};

pub struct World<M> {
    state_controller: StateController<M>,
    current_state: StateEntry,
}

impl World<Pending> {
    pub fn new<S>(initial_state: S) -> Self
    where
        S: State + 'static,
    {
        World {
            state_controller: StateController::new::<S>(),
            current_state: StateEntry(StateID::of::<S>(), Box::new(initial_state)),
        }
    }

    pub fn register<S>(&mut self, state: S)
    where
        S: State + 'static,
    {
        self.state_controller.register(state);
    }

    pub fn finalize(self) -> World<Running> {
        World {
            state_controller: self.state_controller.run(),
            current_state: self.current_state,
        }
    }
}

impl World<Running> {
    pub fn run(&mut self) {
        loop {
            self.current_state.update(&mut self.state_controller);
            if let Some(mut next_state_entry) = self.state_controller.try_update() {
                std::mem::swap(&mut self.current_state, &mut next_state_entry);
                self.state_controller.insert_current_state(next_state_entry);
            }
            self.current_state.render();
        }
    }
}
