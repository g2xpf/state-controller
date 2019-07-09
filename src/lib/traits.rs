use crate::{event::Event, types::Shifter};
use glium::Frame;

pub trait Updatable {
    fn update(&mut self, _state_controller: &mut Shifter) {}
}

pub trait Renderable {
    fn render(&self, frame: &mut Frame) {
        use glium::Surface;

        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.set_finish().unwrap();
    }
}

pub trait EventHandler {
    fn handle(&mut self, _event: &Event) {}
}
