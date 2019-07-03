use crate::{
    traits::{EventHandler, Renderable, Updatable},
    types::StateID,
};
use std::any::Any;

pub trait State: Updatable + Renderable + EventHandler + Any + 'static {
    // run when transitioning to this state
    fn initialize(&mut self) {}

    // run when transitioning to this state
    fn finalize(&mut self) {}
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
