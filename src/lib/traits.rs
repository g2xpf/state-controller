use crate::{event::Event, types::Shifter};
use glium::Frame;
use std::any::{Any, TypeId};

pub trait Updatable {
    fn update(&mut self, _shifter: &mut Shifter) {}
}

pub trait Renderable {
    fn render(&self, _shifter: &Shifter, _frame: &mut Frame) {}
}

pub trait EventHandler {
    fn handle(&mut self, _shifter: &Shifter, _event: &Event) {}
    fn handle_by_ref(&self, _event: &Event) {}
}

pub struct TransitionEvidence;
pub trait Transitionable: Any {
    fn evidence(&self) -> TransitionEvidence;
}

impl dyn Transitionable {
    #[inline]
    pub fn is<T: Transitionable>(&self) -> bool {
        TypeId::of::<T>() == Any::type_id(self)
    }

    #[inline]
    pub fn downcast_ref<T: Transitionable + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Transitionable as *const T)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: Transitionable>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Transitionable as *mut T)) }
        } else {
            None
        }
    }
}
