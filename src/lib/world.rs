use crate::{
    event_controller::EventController,
    intermediate_state::IntermediateState,
    shifter_mode::{Pending, Running},
    state::State,
    state_controller::StateController,
};
use glium::{glutin::EventsLoop, Display};

pub struct World<M> {
    state_controller: StateController<M>,
    event_controller: EventController,
    display: Display,
}

impl World<Pending> {
    pub fn new<S>(events_loop: EventsLoop, display: Display, initial_state: S) -> Self
    where
        S: State + 'static,
    {
        World {
            state_controller: StateController::new::<S>(initial_state),
            event_controller: EventController::new(events_loop),
            display,
        }
    }

    pub fn register_state<S>(mut self, state: S) -> Self
    where
        S: State + 'static,
    {
        self.state_controller.register_state(state);
        self
    }

    pub fn try_register_transition<F, T, I>(mut self, intermediate_state: I) -> Option<Self>
    where
        F: State,
        T: State,
        I: IntermediateState,
    {
        if self
            .state_controller
            .try_register_transition::<F, T, I>(intermediate_state)
        {
            Some(self)
        } else {
            None
        }
    }

    pub fn register_transition<F, T, I>(self, intermediate_state: I) -> Self
    where
        F: State,
        T: State,
        I: IntermediateState,
    {
        self.try_register_transition::<F, T, I>(intermediate_state)
            .unwrap()
    }

    pub fn finalize(self) -> World<Running> {
        World {
            state_controller: self.state_controller.run(),
            event_controller: self.event_controller,
            display: self.display,
        }
    }
}

impl World<Running> {
    pub fn run(&mut self) {
        self.state_controller.initialize();
        loop {
            // event handling
            self.event_controller.poll_events();
            self.state_controller
                .handle_events(&self.event_controller.event);

            // update
            self.state_controller.update();

            // rendering
            let mut frame = self.display.draw();
            self.state_controller.render(&mut frame);
        }
    }
}
