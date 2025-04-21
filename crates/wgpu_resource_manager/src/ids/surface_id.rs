use core::fmt::Display;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

/// The ID of a WGPU surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SurfaceId(u32);

impl SurfaceId {
    /// Generates the next unique `SurfaceId`.
    ///
    /// This function uses an atomic counter to ensure that each generated
    /// `SurfaceId` is unique across threads. Defaults to `0` indicating the
    /// surface is uninitialized.
    ///
    /// # Returns
    /// A new `SurfaceId` with a unique, incremented value.
    pub(crate) fn next() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// A constant function to create an unconfigured [`SurfaceId`].
    #[inline]
    pub const fn default() -> Self {
        Self(0)
    }

    /// Checks if the `SurfaceId` is uninitialized or has a default value.
    ///
    /// # Returns
    /// - `true` if the `SurfaceId` is 0 (uninitialized), otherwise `false`.
    #[allow(dead_code)]
    pub fn is_uninitialized(&self) -> bool {
        self.0 == 0
    }
}

impl Display for SurfaceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Hash for SurfaceId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}