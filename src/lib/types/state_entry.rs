use crate::{
    state::State,
    types::{SharedState, StateID},
};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub(crate) struct StateEntry(pub(crate) StateID, pub(crate) SharedState);

impl StateEntry {
    pub fn new<S>(state: S) -> Self
    where
        S: State + 'static,
    {
        StateEntry(StateID::of::<S>(), Rc::new(RefCell::new(state)))
    }
}

impl Deref for StateEntry {
    type Target = SharedState;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for StateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
