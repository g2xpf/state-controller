pub struct ApplicationEvent {
    pub awakened: bool,
    pub suspended: Option<bool>,
}

impl ApplicationEvent {
    pub fn new() -> Self {
        ApplicationEvent {
            awakened: false,
            suspended: None,
        }
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
