#[derive(Debug, Clone)]
pub struct TimeOfDayData {
    pub time_of_day: f32,
    pub day: f32,
}

impl Default for TimeOfDayData {
    fn default() -> Self {
        let time_of_day = 12.0;
        Self {
            time_of_day,
            day: time_of_day / 24.0,
        }
    }
}
