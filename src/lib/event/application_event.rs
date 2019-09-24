#[derive(Debug, Copy, Clone)]
pub struct ApplicationEvent {
    awakened: bool,
    suspended: Option<bool>,
}

impl ApplicationEvent {
    pub fn new() -> Self {
        ApplicationEvent {
            awakened: false,
            suspended: None,
        }
    }

    pub fn awakened(&self) -> bool {
        self.awakened
    }

    pub fn suspended(&self) -> Option<bool> {
        self.suspended
    }

    pub fn set_awakened(&mut self) {
        self.awakened = true;
    }

    pub fn set_suspended(&mut self, b: bool) {
        self.suspended = Some(b);
    }

    pub fn reset(&mut self) {
        self.awakened = false;
        self.suspended = None;
    }
}
