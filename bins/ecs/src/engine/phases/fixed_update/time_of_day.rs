use crate::engine::{
    ApplyTimeOfDaySystem, CreateSystem, FrameContext, PhysicsSystem, SystemFactory,
    TimeOfDayComponent, TimeOfDaySystem,
};
use std::convert::Infallible;
use std::fmt::Debug;

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
        time_of_day.time_of_day += context.fixed_time_secs;
        time_of_day.time_of_day %= 24.0;
    }
}
