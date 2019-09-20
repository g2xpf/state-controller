use crate::{shifter_mode::Running, state::State, state_shifter::StateShifter};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod key;
pub mod state_entry;

pub type Position = glium::glutin::dpi::LogicalPosition;
pub type Shifter = StateShifter<Running>;
pub type SharedState = Rc<RefCell<dyn State>>;
pub type StateRef<'a, S> = Ref<'a, S>;
pub type StateRefMut<'a, S> = RefMut<'a, S>;
pub(crate) type StateID = std::any::TypeId;
