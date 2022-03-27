#[derive(Debug)]
pub struct WindowEvent {
    pub focused: Option<bool>,
    pub close_requested: bool,
}

impl WindowEvent {
    pub fn new() -> Self {
        WindowEvent {
            focused: None,
            close_requested: false,
        }
    }

    pub fn set_focused(&mut self, b: bool) {
        self.focused = Some(b);
    }

    pub fn set_close_requested(&mut self) {
        self.close_requested = true;
    }

    pub fn reset(&mut self) {
        self.close_requested = false;
        self.focused = None;
    }
}
