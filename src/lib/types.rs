use crate::{controller_mode::Running, state::State, state_controller::StateController};
use std::ops::{Deref, DerefMut};

pub type Shifter = StateController<Running>;
pub(crate) type StateID = std::any::TypeId;
pub(crate) struct StateEntry(pub(crate) StateID, pub(crate) Box<dyn State>);

impl Deref for StateEntry {
    type Target = Box<dyn State>;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for StateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
