use crate::{controller_mode::Running, state_controller::StateController};
use glium::{glutin::Event, Frame};

pub trait Updatable {
    fn update(&mut self, state_controller: &mut StateController<Running>, events: &Vec<Event>);
}

pub trait Renderable {
    fn render(&self, frame: &mut Frame);
}
