use super::easing::Easing;
use std::time::{Duration, Instant};

#[derive(PartialEq, Debug)]
pub enum TimerState {
    Stop,
    Full,
    Counting(f64),
}

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

    pub fn get_ratio(&self) -> TimerState {
        if self.timer.is_none() {
            return TimerState::Stop;
        }

        let current_duration = self.timer.unwrap().elapsed();
        if current_duration > self.limit {
            return TimerState::Full;
        }
        let ratio = current_duration.as_nanos() as f64 / self.limit.as_nanos() as f64;
        TimerState::Counting(ratio)
    }

    pub fn get_ratio_easing<E>(&self) -> TimerState
    where
        E: Easing,
    {
        match self.get_ratio() {
            TimerState::Counting(ratio) => TimerState::Counting(E::fetch(ratio)),
            e @ _ => e,
        }
    }
}
