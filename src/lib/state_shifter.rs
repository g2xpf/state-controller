use crate::{
    shifter_mode::{Pending, Running},
    state::State,
    types::{state_entry::StateEntry, SharedState, StateID},
};

use std::{any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

pub struct StateShifter<M> {
    pub(crate) states: HashMap<StateID, SharedState>,
    pub(crate) next_state: Option<StateEntry>,
    shifter_mode: PhantomData<M>,
}

impl<M> StateShifter<M> {
    pub(crate) fn remove<S>(&mut self) -> Option<SharedState>
    where
        S: State + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.remove(&state_id)
    }

    fn insert<S>(&mut self, state: SharedState)
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

    pub(crate) fn get<S>(&self) -> Option<&SharedState>
    where
        S: State + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.get(&state_id)
    }
}

impl StateShifter<Pending> {
    pub fn new<S>() -> Self
    where
        S: State + 'static,
    {
        StateShifter {
            states: HashMap::new(),
            next_state: None,
            shifter_mode: PhantomData,
        }
    }

    pub fn register<S>(&mut self, state: S)
    where
        S: State + 'static,
    {
        self.insert::<S>(Rc::new(RefCell::new(state)) as Rc<RefCell<dyn State>>);
    }

    pub fn run(self) -> StateShifter<Running> {
        StateShifter {
            states: self.states,
            next_state: None,
            shifter_mode: PhantomData,
        }
    }
}

impl StateShifter<Running> {
    pub(crate) fn try_update(&mut self) -> Option<StateEntry> {
        let next_state = self.next_state.take()?;
        Some(next_state)
    }
}
