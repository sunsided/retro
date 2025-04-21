use core::fmt::Display;
use core::num::NonZeroU32;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};

/// The ID of a WGPU Resource Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WgpuResourceManagerId(NonZeroU32);

impl WgpuResourceManagerId {
    /// Generates the next unique `WgpuResourceManagerId`.
    ///
    /// This function uses an atomic counter to ensure that each generated
    /// `WgpuResourceManagerId` is unique across threads.
    ///
    /// # Returns
    /// A new `WgpuResourceManagerId` with a unique, incremented value.
    pub(crate) fn next() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let next_ud64 = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        WgpuResourceManagerId(NonZeroU32::new(next_ud64).expect("ID is always greater than 0"))
    }
}

impl Display for WgpuResourceManagerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Hash for WgpuResourceManagerId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}