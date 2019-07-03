use crate::{
    controller_mode::{Pending, Running},
    state::State,
    state_controller::StateController,
};
use glium::{
    glutin::{Event, EventsLoop},
    Display,
};

pub struct World<M> {
    state_controller: StateController<M>,
    events_loop: EventsLoop,
    display: Display,
}

impl World<Pending> {
    pub fn new<S>(events_loop: EventsLoop, display: Display, initial_state: S) -> Self
    where
        S: State + 'static,
    {
        World {
            state_controller: StateController::new::<S>(initial_state),
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
            // event handling
            let events = self.get_events();
            self.state_controller.handle_events(&events);

            // update
            self.state_controller.update();

            // rendering
            let mut frame = self.display.draw();
            self.state_controller.render(&mut frame);
        }
    }
}
