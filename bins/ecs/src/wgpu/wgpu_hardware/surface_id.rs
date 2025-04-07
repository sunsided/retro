use std::fmt::Display;

/// The ID of a WGPU surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceId(u64);

impl SurfaceId {
    /// Generates the next unique `SurfaceId`.
    ///
    /// This function uses an atomic counter to ensure that each generated
    /// `SurfaceId` is unique across threads.
    ///
    /// # Returns
    /// A new `SurfaceId` with a unique, incremented value.
    pub fn next() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        SurfaceId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
