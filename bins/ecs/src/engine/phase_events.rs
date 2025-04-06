//! This module defines the `PhaseEvents` structure which is used to measure the
//! duration of different phases in the system startup process. It implements the
//! `SystemStartupPhaseEvents` trait, which provides hooks to execute logic when
//! a phase begins or ends.
//!
//! The timing functionality is built on top of `std::time::Instant`, allowing
//! precise measurements of elapsed time. The elapsed time for a phase is logged
//! using the `tracing` crate for diagnostic purposes.
//!
//! # Example Use
//!
//! The [`SystemRenderPhaseEvents`](crate::engine::SystemRenderPhaseEvents) utilizes the
//! phase begin/end hooks to initialize a render target and commit it per frame.
//!
//! # Key Components
//!
//! - **PhaseEvents**: A structure that keeps track of when a phase starts and calculates
//!   its elapsed time.

use std::time::Instant;

#[derive(Debug)]
pub struct PhaseEvents(Instant);

impl Default for PhaseEvents {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl PhaseEvents {
    /// Resets the start time to the current moment.
    ///
    /// This method should be called to begin measuring the elapsed time
    /// for a specific phase or operation.
    #[inline]
    pub fn start_timing(&mut self) {
        self.0 = Instant::now();
    }

    /// Returns the duration since the start time was last set.
    ///
    /// This method provides the elapsed time since `start_timing` was
    /// called, helping measure the length of an operation or phase.
    #[inline]
    pub fn elapsed(&self) -> std::time::Duration {
        self.0.elapsed()
    }
}
