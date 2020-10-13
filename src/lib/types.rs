use crate::{shifter_mode::Running, state::State, state_shifter::StateShifter};
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod key;
pub mod state_entry;
mod transition_flow;

pub use transition_flow::TransitionFlow;
pub type Position = glium::glutin::dpi::PhysicalPosition<f64>;
pub type Shifter<E = ()> = StateShifter<Running, E>;
pub type SharedState<T> = Rc<RefCell<dyn State<T>>>;
pub type StateRef<'a, S> = Ref<'a, S>;
pub type StateRefMut<'a, S> = RefMut<'a, S>;
pub trait Transitionable {}
pub(crate) type StateID = TypeId;
pub(crate) type IntermediateStateID = (TypeId, TypeId);
