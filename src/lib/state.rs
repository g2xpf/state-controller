use crate::{
    receiver::Receiver,
    shifter_mode::Running,
    state_shifter::StateShifter,
    traits::{EventHandler, Renderable, Updatable},
    types::{state_entry::StateEntry, StateID},
};
use std::any::Any;

pub trait State: Updatable + Renderable + EventHandler + Any + 'static {
    // run when transitioning to this state
    fn initialize(&mut self) {}

    // run when transitioning to this state
    fn finalize(&mut self) {}

    fn shift<S>(&self, shifter: &mut StateShifter<Running>)
    where
        Self: Sized,
        S: State + 'static,
        S: Receiver<Self>,
    {
        if shifter.next_state.is_some() {
            panic!("Cannot set the next state twice");
        }

        // fetch next state
        let next_state = shifter
            .remove::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));

        // set next state
        let next_state_id = StateID::of::<S>();

        shifter.next_state = Some(StateEntry(next_state_id, next_state));
    }

    // shift to the other state
    fn shift_with<S>(&self, shifter: &mut StateShifter<Running>, message: S::Message)
    where
        Self: Sized,
        S: State + 'static,
        S: Receiver<Self>,
    {
        if shifter.next_state.is_some() {
            panic!("Cannot set the next state twice");
        }

        // fetch next state
        let next_state = shifter
            .remove::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));

        // send M from Self to S
        next_state
            .borrow_mut()
            .downcast_mut::<S>()
            .unwrap()
            .receive(message);

        // set next state
        let next_state_id = StateID::of::<S>();

        shifter.next_state = Some(StateEntry(next_state_id, next_state));
    }
}

impl dyn State {
    #[inline]
    pub fn is<S: State>(&self) -> bool {
        StateID::of::<S>() == Any::type_id(self)
    }

    #[inline]
    pub fn downcast_ref<S: State + 'static>(&self) -> Option<&S> {
        if self.is::<S>() {
            unsafe { Some(&*(self as *const dyn State as *const S)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<S: State>(&mut self) -> Option<&mut S> {
        if self.is::<S>() {
            unsafe { Some(&mut *(self as *mut dyn State as *mut S)) }
        } else {
            None
        }
    }
}
