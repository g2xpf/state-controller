use crate::{
    controller_mode::{Pending, Running},
    state::State,
    state_shifter::StateShifter,
    types::StateEntry,
};
use glium::{glutin::Event, Frame};
use std::mem;

pub struct StateController<T> {
    state_shifter: StateShifter<T>,
    current_state: StateEntry,
}

impl StateController<Pending> {
    pub fn new<S>(initial_state: S) -> Self
    where
        S: State + 'static,
    {
        StateController {
            state_shifter: StateShifter::new::<S>(),
            current_state: StateEntry::new(initial_state),
        }
    }

    pub fn register<S>(&mut self, state: S)
    where
        S: State + 'static,
    {
        self.state_shifter.register(state);
    }

    pub fn run(self) -> StateController<Running> {
        StateController {
            state_shifter: self.state_shifter.run(),
            current_state: self.current_state,
        }
    }
}

impl StateController<Running> {
    pub(crate) fn handle_events(&mut self, events: &Vec<Event>) {
        events
            .into_iter()
            .for_each(|ev| self.current_state.handle(&ev));
    }

    pub fn initialize(&mut self) {
        self.current_state.initialize();
    }

    pub fn update(&mut self) {
        self.current_state.update(&mut self.state_shifter);

        if let Some(mut next_state_entry) = self.state_shifter.try_update() {
            self.current_state.finalize();
            next_state_entry.initialize();
            mem::swap(&mut self.current_state, &mut next_state_entry);
            self.state_shifter.insert_current_state(next_state_entry);
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.current_state.render(frame);
    }
}
