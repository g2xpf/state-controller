use crate::{controller_mode::Running, state_controller::StateController};

pub trait Updatable {
    fn update(&mut self, state_controller: &mut StateController<Running>);
}

pub trait Renderable {
    fn render(&self);
}
