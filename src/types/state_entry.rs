use crate::{
    intermediate_state::IntermediateState,
    state::State,
    types::{IntermediateStateID, SharedState, StateID},
};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};
macro_rules! impl_entry_deref {
    ($type: ty, $target: ty) => {
        impl Deref for $type {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.1
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.1
            }
        }
    };
}

pub(crate) struct StateEntry(pub(crate) StateID, pub(crate) SharedState);

impl StateEntry {
    pub fn new<S>(state: Rc<RefCell<S>>) -> Self
    where
        S: State + 'static,
    {
        StateEntry(StateID::of::<S>(), state)
    }
}

impl_entry_deref!(StateEntry, SharedState);

pub(crate) struct IntermediateStateEntry(
    pub(crate) IntermediateStateID,
    pub(crate) Box<dyn IntermediateState>,
);

impl_entry_deref!(IntermediateStateEntry, Box<dyn IntermediateState>);
