#[derive(Debug, Clone)]
pub struct TimeOfDayData {
    pub time_of_day: f32,
}

impl Default for TimeOfDayData {
    fn default() -> Self {
        Self { time_of_day: 12.0 }
    }
}
