use crate::{event::Event, traits::Transitionable, types::TransitionFlow};

use glium::Frame;
// this trait is for constructing trait objects of IntermediateState without specifying associated types

pub trait IntermediateState: 'static {
    fn transition_location(&mut self) -> &mut dyn Transitionable;
    fn update(&mut self) -> TransitionFlow;
    fn render(&self, frame: &mut Frame);
    fn handle(&mut self, event: &Event) -> TransitionFlow;
}
