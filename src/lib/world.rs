use crate::{
    controller_mode::{Pending, Running},
    state::State,
    state_controller::StateController,
    types::{StateEntry, StateID},
};
use glium::{
    glutin::{Event, EventsLoop},
    Display,
};

pub struct World<M> {
    state_controller: StateController<M>,
    current_state: StateEntry,
    events_loop: EventsLoop,
    display: Display,
}

impl World<Pending> {
    pub fn new<S>(events_loop: EventsLoop, display: Display, initial_state: S) -> Self
    where
        S: State + 'static,
    {
        World {
            state_controller: StateController::new::<S>(),
            current_state: StateEntry(StateID::of::<S>(), Box::new(initial_state)),
            events_loop,
            display,
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
            events_loop: self.events_loop,
            display: self.display,
        }
    }
}

impl World<Running> {
    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = vec![];
        self.events_loop.poll_events(|ev| events.push(ev));
        events
    }

    pub fn run(&mut self) {
        loop {
            let events = self.get_events();
            self.current_state
                .update(&mut self.state_controller, &events);
            if let Some(mut next_state_entry) = self.state_controller.try_update() {
                std::mem::swap(&mut self.current_state, &mut next_state_entry);
                self.state_controller.insert_current_state(next_state_entry);
            }
            let mut frame = self.display.draw();
            self.current_state.render(&mut frame);
        }
    }
}
