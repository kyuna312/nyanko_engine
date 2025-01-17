use std::time::{Duration, Instant};

pub struct Time {
    start_time: Instant,
    last_update: Instant,
    delta_time: Duration,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            delta_time: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_update;
        self.last_update = now;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn time_since_startup(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
