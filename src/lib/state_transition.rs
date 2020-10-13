use crate::{
    state::State,
    traits::{TransitionEvidence, Transitionable},
    types::SharedState,
};

use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

fn clone_downcasted_state<S, E>(orig: &SharedState<E>) -> Option<Rc<RefCell<S>>>
where
    S: State<E>,
    E: 'static,
{
    let rc = Rc::clone(orig);
    let ptr = Rc::into_raw(rc) as *const RefCell<dyn State<E>> as *const RefCell<S>;
    Some(unsafe { Rc::from_raw(ptr) })
}

#[derive(Default)]
pub struct Transition<F, T, E = ()> {
    pub(crate) from: Option<Rc<RefCell<F>>>,
    pub(crate) to: Option<Rc<RefCell<T>>>,
    event_marker: PhantomData<E>,
}

impl<F, T, E> Transition<F, T, E>
where
    F: State<E>,
    T: State<E>,
    E: 'static,
{
    pub fn new() -> Self {
        Transition {
            from: None,
            to: None,
            event_marker: PhantomData,
        }
    }

    pub(crate) fn from_states(from: &SharedState<E>, to: &SharedState<E>) -> Self {
        let from = clone_downcasted_state::<F, E>(from);
        let to = clone_downcasted_state::<T, E>(to);
        Transition {
            from,
            to,
            event_marker: PhantomData,
        }
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

impl<F, T, E> Transitionable for Transition<F, T, E>
where
    F: State<E>,
    T: State<E>,
    E: 'static,
{
    fn evidence(&self) -> TransitionEvidence {
        TransitionEvidence
    }
}
