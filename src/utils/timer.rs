use super::easing::Easing;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Timer {
    limit: Duration,
    timer: Option<Instant>,
}

impl Timer {
    pub fn from_secs(limit: u64) -> Self {
        Timer {
            limit: Duration::from_secs(limit),
            timer: None,
        }
    }

    pub fn from_millis(limit: u64) -> Self {
        Timer {
            limit: Duration::from_millis(limit),
            timer: None,
        }
    }

    pub fn from_micors(limit: u64) -> Self {
        Timer {
            limit: Duration::from_micros(limit),
            timer: None,
        }
    }

    pub fn from_nanos(limit: u64) -> Self {
        Timer {
            limit: Duration::from_nanos(limit),
            timer: None,
        }
    }

    pub fn start(&mut self) {
        self.timer = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.timer = None;
    }

    pub fn restart(&mut self) {
        self.start();
    }

    pub fn is_over(&self) -> Option<bool> {
        let timer = self.timer?;
        Some(timer.elapsed() > self.limit)
    }

    pub fn get_ratio(&self) -> Option<f64> {
        let timer = self.timer?;
        let current_duration = timer.elapsed();
        if current_duration > self.limit {
            return Some(1.0);
        }
        let ratio = current_duration.as_nanos() as f64 / self.limit.as_nanos() as f64;
        Some(ratio)
    }

    pub fn get_ratio_easing<E>(&self) -> Option<f64>
    where
        E: Easing,
    {
        self.get_ratio().map(E::fetch)
    }
}
