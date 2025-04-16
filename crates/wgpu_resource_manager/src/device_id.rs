use core::fmt::Display;
use std::sync::atomic::{AtomicU32, Ordering};

/// The ID of a WGPU device.
///
/// Valid device IDs are nonzero internally, and the [`DeviceId::is_uninitialized`](DeviceId::is_uninitialized)
/// function can be used to test whether a device ID was actually provided.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(u32);

impl DeviceId {
    /// Generates the next unique `DeviceId`.
    ///
    /// This function uses an atomic counter to ensure that each generated
    /// `DeviceId` is unique across threads. Defaults to `0` indicating the
    /// device is uninitialized.
    ///
    /// # Returns
    /// A new `DeviceId` with a unique, incremented value.
    pub(crate) fn next() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        DeviceId(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// A constant function to create an unconfigured [`DeviceId`].
    #[inline]
    pub const fn default() -> Self {
        Self(0)
    }

    /// Checks if the `DeviceId` is uninitialized or has a default value.
    ///
    /// # Returns
    /// - `true` if the `DeviceId` is 0 (uninitialized), otherwise `false`.
    #[allow(dead_code)]
    pub fn is_uninitialized(&self) -> bool {
        self.0 == 0
    }
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
