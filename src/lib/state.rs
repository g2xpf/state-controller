use crate::{
    traits::{Renderable, Updatable},
    types::StateID,
};
use std::any::Any;

pub trait State: Updatable + Renderable {}

impl dyn State {
    #[inline]
    pub fn is<T>(&self) -> bool
    where
        T: Any,
    {
        StateID::of::<T>() == State::type_id(self)
    }

    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn State as *const T)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn State as *mut T)) }
        } else {
            None
        }
    }
}
