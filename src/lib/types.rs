use crate::{shifter_mode::Running, state::State, state_shifter::StateShifter};
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod key;
pub(crate) mod pandora_box;
pub mod state_entry;
mod transition_flow;

pub use transition_flow::TransitionFlow;
pub type Position = glium::glutin::dpi::LogicalPosition;
pub type Shifter = StateShifter<Running>;
pub type SharedState = Rc<RefCell<dyn State>>;
pub type StateRef<'a, S> = Ref<'a, S>;
pub type StateRefMut<'a, S> = RefMut<'a, S>;
pub trait Transitionable {}
pub(crate) type StateID = TypeId;
pub(crate) type IntermediateStateID = (TypeId, TypeId);
