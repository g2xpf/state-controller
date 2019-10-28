pub mod error;
mod event;
mod event_controller;
mod shifter_mode;
#[macro_use]
pub mod macros;
mod intermediate_state;
mod parent;
// mod polyshapes;
mod receiver;
mod render_context;
mod shapes;
mod state;
mod state_controller;
mod state_shifter;
mod state_transition;
mod traits;
mod types;
pub mod utils;
mod world;

pub mod primitive_shape {
    // pub use super::polyshapes::Text;
    pub use super::shapes::{Circle, CircleContext};
    pub use super::shapes::{Rectangle, RectangleContext};
}

#[macro_use]
pub extern crate glium;

pub use event::Event;
pub use intermediate_state::IntermediateState;
pub use parent::Parent;
// pub use polyshapes::{PolyShape, PolyShapeContainer};
pub use receiver::Receiver;
pub use render_context::RenderContext;
pub use shapes::Shape;
pub use state::State;
pub use state_transition::Transition;
pub use traits::{EventHandler, Renderable, Transitionable, Updatable};
pub use types::key::Key;
pub use types::Shifter;
pub use types::TransitionFlow;
pub use world::World;
