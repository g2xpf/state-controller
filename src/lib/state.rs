use crate::{
    parent::Parent,
    receiver::Receiver,
    traits::{EventHandler, Renderable, Updatable},
    types::{
        state_entry::{IntermediateStateEntry, StateEntry},
        Shifter, StateID, StateRef, StateRefMut,
    },
};
use std::{
    any::Any,
    cell::{Ref, RefMut},
};

pub trait State: Updatable + Renderable + EventHandler + Any + 'static {
    // run when transitioning to this state
    fn initialize(&mut self) {}

    // run when transitioning to this state
    fn finalize(&mut self) {}

    fn shift<S>(&self, shifter: &mut Shifter)
    where
        Self: Sized,
        S: State + 'static,
        S: Receiver<Self>,
    {
        if shifter.next_state.is_some() {
            panic!("Cannot set the next state twice");
        }

        // create state keys
        let current_state_id = StateID::of::<Self>();
        let next_state_id = StateID::of::<S>();
        let id_pair = (current_state_id, next_state_id);

        // set intermediate state from Self to S if found
        shifter.next_intermediate_state = shifter
            .intermediate_states
            .remove(&id_pair)
            .map(|s| IntermediateStateEntry(id_pair, s));

        // fetch next state
        let next_state = shifter
            .get_cloned::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));

        shifter.next_state = Some(StateEntry(next_state_id, next_state));
    }

    // shift to the other state
    fn shift_with<S>(&self, shifter: &mut Shifter, message: S::Message)
    where
        Self: Sized,
        S: State + 'static,
        S: Receiver<Self>,
    {
        if shifter.next_state.is_some() {
            panic!("Cannot set the next state twice");
        }

        // create state keys
        let current_state_id = StateID::of::<Self>();
        let next_state_id = StateID::of::<S>();
        let id_pair = (current_state_id, next_state_id);

        // set intermediate state from Self to S if found
        shifter.next_intermediate_state = shifter
            .intermediate_states
            .remove(&id_pair)
            .map(|s| IntermediateStateEntry(id_pair, s));

        // fetch next state
        let next_state = shifter
            .get_cloned::<S>()
            .unwrap_or_else(|| panic!("Tried to make a transition to the unregistered state"));

        // send M from Self to S
        next_state
            .borrow_mut()
            .downcast_mut::<S>()
            .unwrap()
            .receive(message);

        shifter.next_state = Some(StateEntry(next_state_id, next_state));
    }

    fn parent_ref<'a, P>(&self, shifter: &'a Shifter) -> StateRef<'a, P>
    where
        Self: Sized,
        P: Parent<Self>,
    {
        let parent = shifter
            .get::<P>()
            .unwrap_or_else(|| panic!("Tried to call the unregistered parent"))
            .borrow();
        Ref::map(parent, |s| s.downcast_ref::<P>().unwrap())
    }

    fn parent_mut<'a, P>(&self, shifter: &'a mut Shifter) -> StateRefMut<'a, P>
    where
        Self: Sized,
        P: Parent<Self>,
    {
        let parent = shifter
            .get::<P>()
            .unwrap_or_else(|| panic!("Tried to call the unregistered parent"))
            .borrow_mut();
        RefMut::map(parent, |s| s.downcast_mut::<P>().unwrap())
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
