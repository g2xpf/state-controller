use crate::types::Position;

pub struct CursorEvent {
    pub position: Position,
    pub entered: bool,
    pub left: bool,
}

impl CursorEvent {
    pub fn new() -> Self {
        CursorEvent {
            position: Position::new(0.0, 0.0),
            entered: false,
            left: false,
        }
    }

    pub fn set_position(&mut self, position: &Position) {
        self.position = *position;
    }

    pub fn set_entered(&mut self) {
        self.entered = true;
    }

    pub fn set_left(&mut self) {
        self.left = true;
    }

    pub fn reset(&mut self) {
        self.entered = false;
        self.left = false;
    }
}
