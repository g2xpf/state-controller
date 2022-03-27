#[macro_use]
extern crate glium;

mod event;
mod event_controller;
mod shifter_mode;
#[macro_use]
pub mod macros;
mod intermediate_state;
mod parent;
mod polyshapes;
mod receiver;
mod renderer;
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
    pub use super::polyshapes::Text;
    pub use super::shapes::{Circle, Rectangle, Texture};
}

pub use event::Event;
pub use intermediate_state::IntermediateState;
pub use parent::Parent;
pub use polyshapes::{PolyShape, PolyShapeContainer};
pub use receiver::Receiver;
pub use renderer::RenderContext;
pub use shapes::{Shape, ShapeContainer};
pub use state::State;
pub use state_transition::Transition;
pub use traits::{EventHandler, Renderable, Transitionable, Updatable};
pub use types::key::Key;
pub use types::Shifter;
pub use types::TransitionFlow;
pub use world::World;
