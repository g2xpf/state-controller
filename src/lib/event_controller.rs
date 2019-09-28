use crate::event::Event;
use glium::glutin;

pub struct EventController {
    events_loop: glutin::EventsLoop,
    pub event: Event,
}

impl EventController {
    pub fn new(events_loop: glutin::EventsLoop) -> Self {
        EventController {
            events_loop,
            event: Event::new(),
        }
    }

    pub fn poll_events(&mut self) {
        let event = &mut self.event;
        event.reset();
        self.events_loop.poll_events(|ev| event.register(&ev));
    }

    pub fn initialize(&mut self) {
        self.event.initialize();
    }
}
