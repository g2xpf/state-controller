use crate::{
    state::State,
    traits::{TransitionEvidence, Transitionable},
    types::{pandora_box::PandoraBox, SharedState},
};

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

fn clone_downcasted_state<S>(orig: &SharedState) -> Option<Rc<RefCell<S>>>
where
    S: State,
{
    let rc = Rc::clone(orig);
    let ptr = Rc::into_raw(rc) as *const RefCell<dyn State> as *const RefCell<S>;
    Some(unsafe { Rc::from_raw(ptr) })
}

#[derive(Default)]
pub struct Transition<F, T> {
    pub(crate) from: Option<Rc<RefCell<F>>>,
    pub(crate) to: Option<Rc<RefCell<T>>>,
}

impl<F, T> Transition<F, T>
where
    F: State,
    T: State,
{
    pub fn new() -> Self {
        Transition {
            from: None,
            to: None,
        }
    }

    pub(crate) fn from_states(from: &SharedState, to: &SharedState) -> Self {
        let from = clone_downcasted_state::<F>(from);
        let to = clone_downcasted_state::<T>(to);
        Transition { from, to }
    }

    pub fn borrow(&self) -> (Ref<F>, Ref<T>) {
        (
            self.from.as_ref().unwrap().borrow(),
            self.to.as_ref().unwrap().borrow(),
        )
    }

    pub fn borrow_mut(&mut self) -> (RefMut<F>, RefMut<T>) {
        (
            self.from.as_mut().unwrap().borrow_mut(),
            self.to.as_mut().unwrap().borrow_mut(),
        )
    }
}

impl<F, T> Transitionable for Transition<F, T>
where
    F: State,
    T: State,
{
    fn evidence(&self) -> TransitionEvidence {
        TransitionEvidence
    }
}
