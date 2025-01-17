use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct DebugStats {
    fps: f32,
    frame_time: Duration,
    memory_usage: usize,
    draw_calls: u32,
}

pub struct DebugSystem {
    frame_times: VecDeque<Duration>,
    last_frame: Instant,
    stats: DebugStats,
    max_samples: usize,
}

impl DebugSystem {
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(100),
            last_frame: Instant::now(),
            stats: DebugStats {
                fps: 0.0,
                frame_time: Duration::from_secs(0),
                memory_usage: 0,
                draw_calls: 0,
            },
            max_samples: 100,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now - self.last_frame;
        self.last_frame = now;

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        // Calculate average frame time and FPS
        let avg_frame_time =
            self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;

        self.stats.frame_time = avg_frame_time;
        self.stats.fps = 1.0 / avg_frame_time.as_secs_f32();

        // Update memory usage (example implementation)
        self.stats.memory_usage = std::mem::size_of::<Self>();
    }

    pub fn get_stats(&self) -> &DebugStats {
        &self.stats
    }

    pub fn set_draw_calls(&mut self, count: u32) {
        self.stats.draw_calls = count;
    }
}
