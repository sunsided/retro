use std::ops::{Add, Mul, Sub};
use std::time::Duration;
use derive_more::Deref;

/// A duration in seconds.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Deref)]
pub struct Seconds(f32);

impl Seconds {
    #[inline]
    pub const fn new(seconds: f32) -> Self {
        Self(seconds)
    }

    #[inline]
    pub const fn as_secs_f32(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Seconds {
    #[inline]
    fn from(seconds: f32) -> Self {
        Self(seconds)
    }
}

impl From<Duration> for Seconds {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self(duration.as_secs_f32())
    }
}

impl From<Seconds> for Duration {
    #[inline]
    fn from(seconds: Seconds) -> Self {
        Duration::from_secs_f32(seconds.0)
    }
}

impl Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, rhs: Seconds) -> Seconds {
        Seconds(self.0 + rhs.0)
    }
}

impl Sub<Seconds> for Seconds {
    type Output = Seconds;

    fn sub(self, rhs: Seconds) -> Self::Output {
        Seconds(self.0 - rhs.0)
    }
}

impl Mul<f32> for Seconds {
    type Output = Seconds;

    fn mul(self, rhs: f32) -> Self::Output {
        Seconds(self.0 * rhs)
    }
}
