use std::time::Instant;
use crate::seconds::Seconds;

/// A simple FPS counter using an exponential moving average filter.
#[derive(Debug, Clone)]
pub struct FpsCounter {
    alpha: f32,
    filtered_frame_time: Option<f32>,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            alpha: 0.2,
            filtered_frame_time: None,
        }
    }
}

impl FpsCounter {
    /// Create a new FPS counter with the given smoothing factor.
    pub fn new(alpha: f32) -> Self {
        FpsCounter {
            alpha,
            filtered_frame_time: None,
        }
    }

    pub fn update_with(&mut self, previous: Instant) -> f32 {
        self.update(previous.elapsed().into())
    }

    /// Update the filtered frame time and return the current FPS.
    pub fn update(&mut self, current_frame_time: Seconds) -> f32 {
        let current_frame_time = current_frame_time.as_secs_f32();
        self.filtered_frame_time = Some(match self.filtered_frame_time {
            // Use EMA formula if previous frame time exists.
            Some(prev) => self.alpha * current_frame_time + (1.0 - self.alpha) * prev,
            // Initialize on the first frame.
            None => current_frame_time,
        });
        let filtered = self.filtered_frame_time.unwrap();
        // Avoid division by zero.
        if filtered > 0.0 {
            1.0 / filtered
        } else {
            0.0
        }
    }
}
