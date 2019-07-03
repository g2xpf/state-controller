use crate::{
    traits::{EventHandler, Renderable, Updatable},
    types::StateID,
};

pub trait State: Updatable + Renderable + EventHandler + 'static {
    fn state_id(&self) -> StateID;
}

impl<S> State for S
where
    S: Updatable + Renderable + EventHandler + 'static,
{
    fn state_id(&self) -> StateID {
        StateID::of::<S>()
    }
}

impl dyn State {
    #[inline]
    pub fn is<S: State>(&self) -> bool {
        StateID::of::<S>() == State::state_id(self)
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
