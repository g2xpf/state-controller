use crate::{
    controller_mode::{Pending, Running},
    receiver::Receiver,
    state::State,
    types::{StateEntry, StateID},
};
use std::{any::TypeId, collections::HashMap, marker::PhantomData};

pub struct StateController<M> {
    states: HashMap<StateID, Box<dyn State>>,
    next_state: Option<StateEntry>,
    controller_mode: PhantomData<M>,
}

impl<M> StateController<M> {
    fn remove<S>(&mut self) -> Option<Box<dyn State>>
    where
        S: State + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.remove(&state_id)
    }

    fn insert<S>(&mut self, state: Box<dyn State>)
    where
        S: State + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.insert(state_id, state);
    }

    pub(crate) fn insert_current_state(&mut self, state_entry: StateEntry) {
        let StateEntry(state_id, state) = state_entry;
        self.states.insert(state_id, state);
    }
}

impl StateController<Pending> {
    pub fn new<S>() -> Self
    where
        S: State + 'static,
    {
        StateController {
            states: HashMap::new(),
            next_state: None,
            controller_mode: PhantomData,
        }
    }

    pub fn register<S>(&mut self, state: S)
    where
        S: State + 'static,
    {
        self.insert::<S>(Box::new(state) as Box<dyn State>);
    }

    pub fn run(self) -> StateController<Running> {
        StateController {
            states: self.states,
            next_state: None,
            controller_mode: PhantomData,
        }
    }
}

impl StateController<Running> {
    pub(crate) fn try_update(&mut self) -> Option<StateEntry> {
        let next_state = self.next_state.take()?;
        Some(next_state)
    }

    pub fn shift<S1, S2>(&mut self, message: <S2 as Receiver<S1>>::Message)
    where
        S1: State + 'static,
        S2: State + 'static,
        S2: Receiver<S1>,
    {
        if self.next_state.is_some() {
            panic!("Cannot set the next state twice");
        }

        // fetch next state
        let mut next_state = self
            .remove::<S2>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));

        // send M from S1 to S2
        next_state
            .as_mut()
            .downcast_mut::<S2>()
            .unwrap()
            .receive(message);

        // set next state
        let next_state_id = StateID::of::<S2>();

        self.next_state = Some(StateEntry(next_state_id, next_state));
    }
}
