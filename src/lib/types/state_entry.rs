use crate::{state::State, types::StateID};
use std::ops::{Deref, DerefMut};

pub(crate) struct StateEntry(pub(crate) StateID, pub(crate) Box<dyn State>);

impl StateEntry {
    pub fn new<S>(state: S) -> Self
    where
        S: State + 'static,
    {
        StateEntry(StateID::of::<S>(), Box::new(state))
    }
}

impl Deref for StateEntry {
    type Target = Box<dyn State>;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for StateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
