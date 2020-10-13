use crate::event::Event;
use crate::shifter_mode::Running;
use crate::state_controller::StateController;
use glium::glutin::event;
use glium::glutin::event_loop::{self};
use glium::{Display, Surface};

pub struct EventController<E: 'static> {
    events_loop: event_loop::EventLoop<E>,
    pub event: Event<E>,
}

impl<E: 'static> EventController<E> {
    pub fn new(events_loop: event_loop::EventLoop<E>) -> Self {
        EventController {
            events_loop,
            event: Event::new(),
        }
    }

    pub fn run(self, mut state_controller: StateController<Running, E>, display: Display) {
        let event_loop = self.events_loop;
        let mut event = self.event;
        event_loop.run(move |raw_event, _, control_flow| match raw_event {
            event::Event::MainEventsCleared => {
                state_controller.handle_events(&event);
                event.initialize();
                *control_flow = state_controller.update();
                display.gl_window().window().request_redraw();
            }
            event::Event::RedrawRequested(_) => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                state_controller.render(&mut frame);
                frame.finish().unwrap();
            }
            e => {
                event.register(&e);
            }
        });
    }
}
