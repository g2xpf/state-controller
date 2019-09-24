use crate::types::key::{virtual_keycode_to_key, Key};
use glium::glutin;
use std::fmt;

const CODE_KINDS: usize = 161;

#[derive(Default, Copy, Clone, Debug)]
pub struct KeyEntry {
    pressed: bool,
    up_times: usize,
    down_times: usize,
}

impl KeyEntry {
    pub fn new() -> Self {
        KeyEntry {
            pressed: false,
            up_times: 0,
            down_times: 0,
        }
    }

    pub fn reset(&mut self) {
        self.up_times = 0;
        self.down_times = 0;
    }

    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn is_up(&self) -> bool {
        self.up_times > 0
    }

    pub fn is_down(&self) -> bool {
        self.down_times > 0
    }

    pub fn up_times(&self) -> usize {
        self.up_times
    }

    pub fn down_times(&self) -> usize {
        self.down_times
    }
}

pub struct KeyEvent {
    keys: [KeyEntry; CODE_KINDS],
    text: String,
    changed: Vec<Key>,
}

impl fmt::Debug for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(
                self.changed
                    .iter()
                    .map(|key| (key, self.keys[*key as usize])),
            )
            .finish()
    }
}

impl KeyEvent {
    pub fn new() -> Self {
        KeyEvent {
            keys: [KeyEntry::new(); CODE_KINDS],
            text: String::new(),
            changed: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        for key_code in self.changed.drain(..) {
            let key_entry = &mut self.keys[key_code as usize];
            key_entry.reset();
        }
    }

    pub fn key(&self, key: Key) -> &KeyEntry {
        &self.keys[key as usize]
    }

    pub fn register_key(&mut self, keyboard_input: &glutin::KeyboardInput) {
        if let Some(key_code) = keyboard_input.virtual_keycode {
            self.changed.push(virtual_keycode_to_key(key_code));
            let key_entry = &mut self.keys[key_code as usize];

            match keyboard_input.state {
                glutin::ElementState::Pressed => {
                    if !key_entry.pressed {
                        key_entry.down_times += 1;
                    }
                    key_entry.pressed = true;
                }
                glutin::ElementState::Released => {
                    if key_entry.pressed {
                        key_entry.up_times += 1;
                    }
                    key_entry.pressed = false;
                }
            }
        }
    }

    pub fn register_text(&mut self, c: char) {
        self.text.push(c);
    }
}
