#[macro_use]
extern crate glium;

mod event;
mod event_controller;
mod shifter_mode;
#[macro_use]
pub mod macros;
mod receiver;
mod renderer;
mod shapes;
mod state;
mod state_controller;
mod state_shifter;
mod traits;
mod types;
pub mod utils;
mod world;

pub use event::Event;
pub use receiver::Receiver;
pub use renderer::RenderContext;
pub use shapes::{primitive_shape, Shape, ShapeContainer};
pub use state::State;
pub use traits::{EventHandler, Renderable, Updatable};
pub use types::key::Key;
pub use types::Shifter;
pub use world::World;
