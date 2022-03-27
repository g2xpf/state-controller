use crate::types::Position;

#[derive(Debug, Copy, Clone)]
pub struct CursorEvent {
    position: Position,
    entered: bool,
    left: bool,
}

impl CursorEvent {
    pub fn new() -> Self {
        CursorEvent {
            position: Position::new(0.0, 0.0),
            entered: false,
            left: false,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn is_entered(&self) -> bool {
        self.entered
    }

    pub fn is_left(&self) -> bool {
        self.left
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
