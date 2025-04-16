use std::convert::Infallible;
use crate::engine::{FrameContext, PhaseEvents, SystemWgpuReinitPhaseEvents};

pub mod shaders;
pub mod textures;

impl SystemWgpuReinitPhaseEvents for PhaseEvents {
    type Result = ();
    type Error = Infallible;

    fn on_begin_phase(&mut self, _context: &FrameContext) -> Result<Self::Result, Self::Error> {
        Ok(())
    }

    fn on_end_phase(&mut self, _result: Option<Self::Result>, _context: &FrameContext) -> Result<(), Self::Error> {
        Ok(())
    }
}
