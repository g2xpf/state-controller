use crate::{
    event_controller::EventController,
    intermediate_state::IntermediateState,
    shifter_mode::{Pending, Running},
    state::State,
    state_controller::StateController,
};
use glium::{glutin::event_loop::EventLoop, Display};

pub struct World<M, E: 'static> {
    state_controller: StateController<M, E>,
    event_controller: EventController<E>,
    display: Display,
}

impl<E> World<Pending, E>
where
    E: 'static,
{
    pub fn new<S>(events_loop: EventLoop<E>, display: Display, initial_state: S) -> Self
    where
        S: State<E> + 'static,
    {
        World {
            state_controller: StateController::new::<S>(initial_state),
            event_controller: EventController::new(events_loop),
            display,
        }
    }

    pub fn register_state<S>(mut self, state: S) -> Self
    where
        S: State<E> + 'static,
    {
        self.state_controller.register_state(state);
        self
    }

    pub fn try_register_transition<F, T, I>(mut self, intermediate_state: I) -> Option<Self>
    where
        F: State<E>,
        T: State<E>,
        I: IntermediateState<E>,
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
        F: State<E>,
        T: State<E>,
        I: IntermediateState<E>,
    {
        self.try_register_transition::<F, T, I>(intermediate_state)
            .unwrap()
    }

    pub fn finalize(self) -> World<Running, E> {
        World {
            state_controller: self.state_controller.run(),
            event_controller: self.event_controller,
            display: self.display,
        }
    }
}

impl<E: 'static> World<Running, E> {
    pub fn run(mut self) {
        self.state_controller.initialize();
        self.event_controller
            .run(self.state_controller, self.display);
    }
}
