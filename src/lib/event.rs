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

use glium::glutin;

#[derive(Debug)]
pub struct Event {
    key: KeyEvent,
    pub cursor: CursorEvent,
    pub motion: MotionEvent,
    pub window: WindowEvent,
    pub app: ApplicationEvent,
}

impl Event {
    pub fn new() -> Self {
        Event {
            key: KeyEvent::new(),
            cursor: CursorEvent::new(),
            motion: MotionEvent::new(),
            window: WindowEvent::new(),
            app: ApplicationEvent::new(),
        }
    }

    pub fn key(&self, key: Key) -> &KeyEntry {
        &self.key.key(key)
    }

    pub fn reset(&mut self) {
        self.key.reset();
        self.cursor.reset();
        self.motion.reset();
        self.window.reset();
        self.app.reset();
    }

    pub fn register(&mut self, event: &glutin::Event) {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                // key events
                glutin::WindowEvent::ReceivedCharacter(c) => {
                    self.key.register_text(*c);
                }

                // cursor events
                glutin::WindowEvent::CursorEntered { .. } => {
                    self.cursor.set_entered();
                }
                glutin::WindowEvent::CursorLeft { .. } => {
                    self.cursor.set_left();
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    self.cursor.set_position(position);
                }

                // window events
                glutin::WindowEvent::CloseRequested => {
                    self.window.set_close_requested();
                }
                _ => (),
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                // key events
                glutin::DeviceEvent::Key(keyboard_input) => {
                    self.key.register_key(keyboard_input);
                }

                // motion events
                glutin::DeviceEvent::MouseMotion { delta } => {
                    self.motion.register_motion(delta);
                }
                _ => (),
            },

            // application events
            glutin::Event::Awakened => {
                self.app.set_awakened();
            }
            glutin::Event::Suspended(b) => {
                self.app.set_suspended(*b);
            }
        }
    }
}
