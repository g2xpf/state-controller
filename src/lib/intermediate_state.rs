use crate::{
    event::Event,
    traits::Transitionable,
    types::{Shifter, TransitionFlow},
};

use glium::Frame;
// this trait is for constructing trait objects of IntermediateState without specifying associated types

pub trait IntermediateState<E = ()>: 'static {
    fn transition_location(&mut self) -> &mut dyn Transitionable;
    fn initialize(&mut self) {}
    fn finalize(&mut self) {}
    fn update(&mut self) -> TransitionFlow {
        TransitionFlow::Break
    }
    fn render(&self, _shifter: &Shifter<E>, _frame: &mut Frame) {}
    fn handle(&mut self, _event: &Event<E>) -> TransitionFlow {
        TransitionFlow::Break
    }
}
