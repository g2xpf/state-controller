pub struct MotionEvent {
    delta: (f64, f64),
}

impl MotionEvent {
    pub fn new() -> Self {
        MotionEvent { delta: (0.0, 0.0) }
    }

    pub fn reset(&mut self) {
        self.delta = (0.0, 0.0);
    }

    pub fn register_motion(&mut self, motion: &(f64, f64)) {
        self.delta.0 += motion.0;
        self.delta.1 += motion.1;
    }
}
