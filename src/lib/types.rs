use crate::{shifter_mode::Running, state_shifter::StateShifter};

pub mod key;
pub mod state_entry;

pub type Position = glium::glutin::dpi::LogicalPosition;
pub type Shifter = StateShifter<Running>;
pub(crate) type StateID = std::any::TypeId;
