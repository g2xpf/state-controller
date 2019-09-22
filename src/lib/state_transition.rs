use crate::{
    state::State,
    traits::{TransitionEvidence, Transitionable},
    types::{pandora_box::PandoraBox, SharedState},
};

use std::{cell::RefCell, rc::Rc};

fn clone_downcasted_state<S>(orig: &SharedState) -> PandoraBox<Rc<RefCell<S>>>
where
    S: State,
{
    let rc = Rc::clone(orig);
    let ptr = Rc::into_raw(rc) as *const RefCell<dyn State> as *const RefCell<S>;
    PandoraBox::new(unsafe { Rc::from_raw(ptr) })
}

#[derive(Default)]
pub struct Transition<F, T> {
    pub from: PandoraBox<Rc<RefCell<F>>>,
    pub to: PandoraBox<Rc<RefCell<T>>>,
}

impl<F, T> Transition<F, T>
where
    F: State,
    T: State,
{
    pub(crate) fn new(from: &SharedState, to: &SharedState) -> Self {
        let from = clone_downcasted_state::<F>(from);
        let to = clone_downcasted_state::<T>(to);
        Transition { from, to }
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
