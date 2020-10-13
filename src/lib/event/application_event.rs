#[derive(Debug, Copy, Clone)]
pub struct ApplicationEvent {
    awakened: bool,
    suspended: bool,
}

impl ApplicationEvent {
    pub fn new() -> Self {
        ApplicationEvent {
            awakened: false,
            suspended: false,
        }
    }

    pub fn awakened(&self) -> bool {
        self.awakened
    }

    pub fn suspended(&self) -> bool {
        self.suspended
    }

    pub fn set_awakened(&mut self) {
        self.awakened = true;
    }

    pub fn set_suspended(&mut self) {
        self.suspended = true;
    }

    pub fn reset(&mut self) {
        self.awakened = false;
        self.suspended = false;
    }
}
