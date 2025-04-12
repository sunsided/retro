use crate::engine::{
    ApplyTimeOfDaySystem, CreateSystem, FrameContext, PhysicsSystem, SystemFactory,
    TimeOfDayComponent, TimeOfDaySystem,
};
use crate::logic::Weekday;
use std::convert::Infallible;
use std::fmt::Debug;
use tracing::debug;

#[derive(Debug, Default)]
pub struct TimeOfDaySystemData;

impl Default for TimeOfDaySystem {
    fn default() -> Self {
        Self(TimeOfDaySystemData)
    }
}

impl CreateSystem<TimeOfDaySystem> for SystemFactory {
    fn create(&self) -> TimeOfDaySystem {
        TimeOfDaySystem::default()
    }
}

impl ApplyTimeOfDaySystem for TimeOfDaySystem {
    type Error = Infallible;

    fn apply_single(&mut self, context: &FrameContext, time_of_day: &mut TimeOfDayComponent) {
        // Loops through one day in 24 seconds.
        const HOURS_PER_DAY: f32 = 24.0; // hours
        const HOURS_PER_SECOND: f32 = 1.0;
        const DAYS_PER_SECOND: f32 = HOURS_PER_DAY / HOURS_PER_SECOND;

        let time_increase = context.fixed_time_secs / HOURS_PER_SECOND;
        let day_increase = context.fixed_time_secs / DAYS_PER_SECOND;

        time_of_day.time_of_day += time_increase;
        time_of_day.time_of_day %= 24.0;

        let next_day = time_of_day.day + day_increase;
        if next_day.floor() > time_of_day.day.floor() {
            debug!(
                "It is now {:?} - {} day{} since the journey began",
                Weekday::from(next_day),
                next_day as u32,
                if next_day as u32 > 1 { "s" } else { "" }
            );
        }

        time_of_day.day = next_day;
    }
}
