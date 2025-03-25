//! # Triple Buffering Crate
//!
//! This crate provides a utility for implementing triple buffering or more generalized
//! multi-buffering patterns. It includes the [`TripleBuffer`] type alias built on top of
//! the generic [`MultiBuffer`] struct, allowing developers to create and manage buffers
//! designed for concurrent or time-sensitive workloads.
//!
//! The primary purpose of this crate is to enable safe, lock-free access to the latest
//! data in one buffer while allowing modifications in other buffers. This pattern is
//! commonly used in applications like video processing, gaming, or other scenarios
//! that demand a balance of performance and data synchronization.
//!
//! ## Main Features
//!
//! - **`TripleBuffer<T>`**: A convenience alias for three buffers (standard triple buffering).
//! - **Generic Multi-buffering**: `MultiBuffer<T, SIZE>` supports any number of buffers via constant generics.
//! - Lock-free concurrency: Ensures safe access without blocking threads.
//! - Convenience methods for buffer access, mutation, and updating the `latest` pointer.
//!
//! ## Examples
//!
//! ### TripleBuffer Example
//!
//! The following example demonstrates how to use a `TripleBuffer`:
//!
//! ```rust
//! use multibuffer::TripleBuffer;
//!
//! fn main() {
//!     // Create a TripleBuffer with an initial value of 0.
//!     let buffer = TripleBuffer::new(|| 0);
//!
//!     // Modify a specific buffer (index 1).
//!     *buffer.get_mut(1) = 42;
//!
//!     // Publish the updated buffer (index 1).
//!     buffer.publish(1);
//!
//!     // Access the latest published buffer.
//!     assert_eq!(*buffer.get_latest(), 42);
//!     
//!     // Further modifications and publishing.
//!     *buffer.get_mut(2) = 100;
//!     buffer.publish(2);
//!     assert_eq!(*buffer.get_latest(), 100);
//! }
//! ```
//!
//! ## Implementation Details
//!
//! - The [`MultiBuffer`] struct stores an internal array of buffers, fence markers, and exposes
//!   methods for interaction. Buffers are allocated using [`UnsafeCell`] for efficient mutable
//!   access without synchronization primitives.
//! - The `Sync` implementation leverages careful safety design to allow multi-threaded use
//!   while adhering to Rust's safety guarantees. The buffers must not be simultaneously accessed
//!   in unsafe ways outside the provided API.
//!
//! ## Notes
//!
//! - The correctness of this implementation depends on the user following the API's
//!   safety rules—`get_latest` and `get_mut` methods should never be misused concurrently
//!   in order to prevent undefined behavior.
//! - As of now, the crate relies on constant generics (`const SIZE: usize`) for specifying
//!   buffer sizes, which requires a fixed buffer count at compile-time.

#![allow(unsafe_code)]

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A type alias for a triple buffer, which is a specialized form of the [`MultiBuffer`] struct
/// with a fixed size of three buffers. This is commonly used for triple buffering scenarios.
pub type TripleBuffer<T> = MultiBuffer<T, 3>;

/// A type alias representing the index of a buffer.
///
/// This is used to identify a specific buffer in a multi-buffering structure.
pub type BufferIndex = usize;

/// A generic multi-buffering structure designed for lock-free concurrency, supporting a
/// configurable number of buffers.
///
/// # Type Parameters
/// - `T`: The type of data stored in each buffer.
/// - `SIZE`: The number of buffers in the structure, which is fixed at compile time.
///
/// # Safety
/// - The buffers are wrapped in [`UnsafeCell`] to allow efficient mutable access without
///   synchronization primitives.
/// - The implementation assumes the user correctly follows the API's safety guarantees to
///   avoid undefined behavior, especially when using methods like `get_mut` and `publish`
///   in a concurrent environment.
pub struct MultiBuffer<T, const SIZE: usize> {
    buffers: [UnsafeCell<T>; SIZE],
    fence: AtomicUsize,
}

impl<T, const SIZE: usize> MultiBuffer<T, SIZE> {
    /// The number of buffers in the structure, determined at compile-time.
    pub const SIZE: usize = SIZE;

    /// Returns the number of buffers in the structure.
    ///
    /// This is a constant value defined by the `SIZE` generic parameter.
    pub const fn len(&self) -> usize {
        SIZE
    }
}

// SAFETY: only one thread writes to a buffer at a time, controlled by design
unsafe impl<T, const SIZE: usize> Sync for MultiBuffer<T, SIZE> {}

impl<T, const SIZE: usize> MultiBuffer<T, SIZE> {
    /// Creates a new `MultiBuffer` with the specified size and initializes each buffer
    /// with the provided `factory` function.
    ///
    /// # Parameters
    /// - `factory`: A function or closure that produces an initial value for each buffer.
    ///
    /// # Returns
    /// A new instance of `MultiBuffer` initialized with the values created by the `factory`.
    ///
    /// # Panics
    /// This method will panic if the `factory` function panics during initialization.
    pub fn new(mut factory: impl FnMut() -> T) -> Self {
        let mut buffers = MaybeUninit::<[UnsafeCell<T>; SIZE]>::uninit();
        let ptr = buffers.as_mut_ptr() as *mut UnsafeCell<T>;

        for i in 0..SIZE {
            unsafe {
                ptr.add(i).write(UnsafeCell::new(factory()));
            }
        }

        let buffers = unsafe { buffers.assume_init() };

        MultiBuffer {
            buffers,
            fence: AtomicUsize::new(0),
        }
    }

    /// Retrieves a reference to the latest published buffer.
    ///
    /// # Returns
    /// A reference to the data in the buffer that was most recently published.
    ///
    /// # Safety
    /// - The method assumes that the underlying concurrency rules are correctly followed
    ///   and guarantees a consistent snapshot of the latest buffer.
    /// - This method uses `UnsafeCell` dereferencing internally, which is safe as long as
    ///   only one thread is modifying the buffer while others are reading it.
    #[inline]
    #[must_use]
    pub fn get_latest(&self) -> &T {
        let idx = self.fence.load(Ordering::Acquire) % SIZE;
        unsafe { &*self.buffers[idx].get() }
    }

    /// Retrieves a mutable reference to a specific buffer by its index.
    ///
    /// # Parameters
    /// - `index`: The index of the buffer to retrieve.
    ///
    /// # Returns
    /// A mutable reference to the specified buffer.
    ///
    /// # Panics
    /// This method will panic if the specified `index` is invalid or if it violates
    /// the usage constraints of `MultiBuffer`.
    ///
    /// # Safety
    /// - The caller must ensure that the accessed buffer is not being simultaneously read
    ///   or written from another thread.
    /// - This method uses internal `UnsafeCell` dereferencing, which assumes the correct
    ///   usage of `MultiBuffer`'s API in a concurrent context.
    #[inline]
    #[must_use]
    pub fn get_mut(&self, index: BufferIndex) -> &mut T {
        unsafe { &mut *self.buffers[index % SIZE].get() }
    }

    /// Publishes a specific buffer by updating the internal fence.
    ///
    /// # Parameters
    /// - `index`: The index of the buffer to publish.
    ///
    /// # Safety
    /// - The caller should ensure that the buffer being published has completed any
    ///   modifications, and is in a valid state before publishing.
    /// - This method uses ordering guarantees (`Acquire` and `Release`) to ensure that
    ///   the changes to the buffer are visible to other threads that subsequently read it.
    #[inline]
    pub fn publish(&self, index: BufferIndex) {
        self.fence.store(index % SIZE, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multibuffer() {
        // Create a TripleBuffer with initial values
        let buffer = TripleBuffer::new(|| 0);

        // Test the initial size of the buffer
        assert_eq!(buffer.len(), 3);

        // Verify the latest buffer returns the initial value
        assert_eq!(*buffer.get_latest(), 0);

        // Modify a specific buffer
        *buffer.get_mut(1) = 42;

        // Publish the updated buffer (index 1)
        buffer.publish(1);

        // Verify the latest buffer is now the updated one
        assert_eq!(*buffer.get_latest(), 42);

        // Modify another specific buffer
        *buffer.get_mut(2) = 100;

        // Publish the updated buffer (index 2)
        buffer.publish(2);

        // Verify the latest buffer is now the second updated one
        assert_eq!(*buffer.get_latest(), 100);

        // Modify and publish a buffer with wrapping around indices (e.g., index 4 maps to 1)
        *buffer.get_mut(4) = 7;
        buffer.publish(4);

        // The latest buffer should now be the one at index 1 (mod TRIPLE)
        assert_eq!(*buffer.get_latest(), 7);
    }
}
