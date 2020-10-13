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
    ($type: tt, $target: ty) => {
        impl<T> Deref for $type<T> {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.1
            }
        }

        impl<T> DerefMut for $type<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.1
            }
        }
    };
}

pub(crate) struct StateEntry<T>(pub(crate) StateID, pub(crate) SharedState<T>);

impl<T: 'static> StateEntry<T> {
    pub fn new<S>(state: Rc<RefCell<S>>) -> Self
    where
        S: State<T>,
    {
        StateEntry(StateID::of::<S>(), state)
    }
}

impl_entry_deref!(StateEntry, SharedState<T>);

pub(crate) struct IntermediateStateEntry<T>(
    pub(crate) IntermediateStateID,
    pub(crate) Box<dyn IntermediateState<T>>,
);

impl_entry_deref!(IntermediateStateEntry, Box<dyn IntermediateState<T>>);
