mod application_event;
mod cursor_event;
mod key_event;
mod motion_event;
mod window_event;

use application_event::ApplicationEvent;
use cursor_event::CursorEvent;
use key_event::{KeyEntry, KeyEvent};
use motion_event::MotionEvent;
use window_event::WindowEvent;

use crate::types::key::Key;

use glium::glutin::event;

#[derive(Debug)]
pub struct Event<T = ()> {
    key: KeyEvent,
    pub cursor: CursorEvent,
    pub motion: MotionEvent,
    pub window: WindowEvent,
    pub app: ApplicationEvent,
    pub custom: Vec<T>,
}

impl<T> Event<T> {
    pub fn new() -> Self {
        Event {
            key: KeyEvent::new(),
            cursor: CursorEvent::new(),
            motion: MotionEvent::new(),
            window: WindowEvent::new(),
            app: ApplicationEvent::new(),
            custom: vec![],
        }
    }

    pub fn key(&self, key: Key) -> &KeyEntry {
        &self.key.key(key)
    }

    pub fn text(&self) -> &str {
        &self.key.text
    }

    pub fn reset(&mut self) {
        self.key.reset();
        self.cursor.reset();
        self.motion.reset();
        self.window.reset();
        self.app.reset();
    }

    pub fn register(&mut self, event: &event::Event<'_, T>) {
        match event {
            event::Event::WindowEvent { event, .. } => {
                match event {
                    // key events
                    event::WindowEvent::ReceivedCharacter(c) => {
                        self.key.register_text(*c);
                    }
                    event::WindowEvent::KeyboardInput { input, .. } => {
                        self.key.register_key(input);
                    }

                    // cursor events
                    event::WindowEvent::CursorEntered { .. } => {
                        self.cursor.set_entered();
                    }
                    event::WindowEvent::CursorLeft { .. } => {
                        self.cursor.set_left();
                    }
                    event::WindowEvent::CursorMoved { position, .. } => {
                        self.cursor.set_position(position);
                    }

                    // window events
                    event::WindowEvent::CloseRequested => {
                        self.window.set_close_requested();
                    }
                    _ => (),
                }
            }
            event::Event::DeviceEvent { event, .. } => match event {
                // motion events
                event::DeviceEvent::MouseMotion { delta } => {
                    self.motion.register_motion(delta);
                }
                _ => (),
            },

            // application events
            event::Event::Resumed => {
                self.app.set_awakened();
            }
            event::Event::Suspended => {
                self.app.set_suspended();
            }
            _ => {}
        }
    }

    pub(crate) fn initialize(&mut self) {
        self.key.initialize();
    }
}
