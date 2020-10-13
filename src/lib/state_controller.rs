use crate::{
    event::Event,
    intermediate_state::IntermediateState,
    shifter_mode::{Pending, Running},
    state::State,
    state_shifter::StateShifter,
    types::{
        state_entry::{IntermediateStateEntry, StateEntry},
        TransitionFlow,
    },
};
use glium::glutin::event_loop::ControlFlow;
use glium::Frame;
use std::{cell::RefCell, mem, rc::Rc};

pub struct StateController<M, E> {
    state_shifter: StateShifter<M, E>,
    current_state: StateEntry<E>,
    current_intermediate_state: Option<IntermediateStateEntry<E>>,
}

impl<E: 'static> StateController<Pending, E> {
    pub fn new<S>(initial_state: S) -> Self
    where
        S: State<E>,
    {
        let initial_state = Rc::new(RefCell::new(initial_state));
        StateController {
            state_shifter: StateShifter::new::<S>(Rc::clone(&initial_state)),
            current_state: StateEntry::new(Rc::clone(&initial_state)),
            current_intermediate_state: None,
        }
    }

    pub fn register_state<S>(&mut self, state: S)
    where
        S: State<E>,
    {
        self.state_shifter
            .register_state(Rc::new(RefCell::new(state)));
    }

    pub fn try_register_transition<F, T, I>(&mut self, intermediate_state: I) -> bool
    where
        F: State<E>,
        T: State<E>,
        I: IntermediateState<E>,
    {
        self.state_shifter
            .try_register_transition::<F, T, I>(intermediate_state)
    }

    pub fn run(self) -> StateController<Running, E> {
        StateController {
            state_shifter: self.state_shifter.run(),
            current_state: self.current_state,
            current_intermediate_state: None,
        }
    }
}

impl<E: 'static> StateController<Running, E> {
    pub(crate) fn handle_events(&mut self, event: &Event<E>) {
        if let Some(IntermediateStateEntry(_, ref mut intermediate_state)) =
            self.current_intermediate_state
        {
            intermediate_state.handle(event);
        } else {
            self.current_state.borrow_mut().handle(&event);
        }
    }

    pub fn initialize(&mut self) {
        self.current_state.borrow_mut().initialize();
    }

    pub fn update(&mut self) -> ControlFlow {
        if let Some(IntermediateStateEntry(_, ref mut intermediate_state)) =
            self.current_intermediate_state
        {
            match intermediate_state.update() {
                TransitionFlow::Break => {
                    let mut intermediate_state = self.current_intermediate_state.take().unwrap();
                    intermediate_state.finalize();
                    self.state_shifter
                        .insert_intermediate_state_entry(intermediate_state);
                }
                TransitionFlow::Exit => return ControlFlow::Exit,
                TransitionFlow::Continue => {}
            }
        }

        self.current_state
            .borrow_mut()
            .update(&mut self.state_shifter);

        if let Some(mut next_state_entry) = self.state_shifter.try_take_next() {
            self.current_state.borrow_mut().finalize();
            next_state_entry.borrow_mut().initialize();
            mem::swap(&mut self.current_state, &mut next_state_entry);
            self.state_shifter.insert_state_entry(next_state_entry);
        }

        if let Some(intermediate_state) = self.state_shifter.next_intermediate_state.take() {
            self.current_intermediate_state = Some(intermediate_state);
        }

        ControlFlow::default()
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if let Some(IntermediateStateEntry(_, ref intermediate_state)) =
            self.current_intermediate_state
        {
            intermediate_state.render(&self.state_shifter, frame);
        } else {
            self.current_state
                .borrow_mut()
                .render(&self.state_shifter, frame);
        }
    }
}
