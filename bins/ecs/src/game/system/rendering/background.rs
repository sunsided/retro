//! This module provides functionality for handling background colors and applying them
//! in a rendering pipeline. It includes definitions for the `BackgroundColorData` struct,
//! which represents RGBA color components, and implements a system for rendering
//! background colors using the wgpu render framework.
//!
//! # Overview
//!
//! - **BackgroundColorData**: A struct to represent background color data with red, green,
//!   blue, and alpha components (`r`, `g`, `b`, `a`). Includes a method to convert the
//!   struct into a `wgpu::Color` for rendering.
//! - **RenderBackgroundSystem**: A system that applies background colors to the current
//!   frame's render target. It clears the existing frame content and replaces it with the
//!   specified background color.
//!
//! This module primarily targets applications using wgpu to manage GPU rendering operations.
//!
//! # Components
//!
//! - **`BackgroundColorData`**:
//!   - Properties: `r`, `g`, `b`, `a` (all in the range `[0.0, 1.0]`).
//!   - Default Color: Dark blue shade (`r = 0.1`, `g = 0.2`, `b = 0.3`, `a = 1.0`).
//!   - Conversion method: `as_wgpu_color()` transforms it into a `wgpu::Color` for render passes.
//!
//! - **`RenderBackgroundSystem`**:
//!   - Applies the `BackgroundColorComponent` to the `WgpuRenderState` during render passes.
//!   - Uses `wgpu::LoadOp::Clear` to replace the current frame with the specified color.
//!
//! # Systems
//!
//! The `ApplyRenderBackgroundSystem` trait is implemented by the `RenderBackgroundSystem`
//! to manage background color rendering. This system ensures that the background is cleared
//! and updated for each render frame.
//!
//! # Usage
//!
//! ```rust
//! use crate::game::component::BackgroundColorComponent;
//! use crate::game::system::{ApplyRenderBackgroundSystem, RenderBackgroundSystem};
//!
//! fn main() {
//!     let mut render_system = RenderBackgroundSystem;
//!     let mut wgpu_state = WgpuRenderState::new(); // Placeholder for actual WgpuRenderState initialization
//!     let background = BackgroundColorComponent::default();
//!
//!     // Apply the background color to the render pipeline.
//!     render_system.apply_single(&mut wgpu_state, &background);
//! }
//! ```

use std::convert::Infallible;
use crate::game::component::BackgroundColorComponent;
use crate::game::system::{ApplyRenderBackgroundSystem, RenderBackgroundSystem};
use crate::game::world::WgpuRenderState;

/// Represents a background color with red, green, blue, and alpha components.
///
/// Each component is a floating-point value in the range [0.0, 1.0], where:
/// - `r`: The red component
/// - `g`: The green component
/// - `b`: The blue component
/// - `a`: The alpha (transparency) component
#[derive(Debug, Clone)]
pub struct BackgroundColorData {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for BackgroundColorData {
    fn default() -> Self {
        Self {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        }
    }
}

impl BackgroundColorData {
    /// Converts the `BackgroundColorData` into a `wgpu::Color`.
    ///
    /// Each component (`r`, `g`, `b`, `a`) is converted from `f32` to `f64`
    /// to match the `wgpu::Color` requirements. This method can be used to
    /// set colors in wgpu render operations.
    ///
    /// # Returns
    /// A `wgpu::Color` instance representing the background color.
    pub const fn as_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}

#[derive(Debug)]
pub struct RenderBackgroundSystemData;

impl ApplyRenderBackgroundSystem for RenderBackgroundSystem {
    /// The error type returned by the system. Infallible means this system cannot fail.
    type Error = Infallible;

    /// Applies the background color to the render pipeline for a single entity.
    ///
    /// This method clears the current frame's render target with the given
    /// `BackgroundColorComponent`. The background color is extracted and
    /// converted into a `wgpu::Color`, which is used during the render pass.
    ///
    /// # Parameters
    /// - `wgpu_render`: A mutable reference to the `WgpuRenderState`, which
    /// contains the current frame's state, including the `CommandEncoder`
    /// and `TextureView`.
    /// - `background_color`: A reference to the `BackgroundColorComponent`
    /// that specifies the clear color to apply.
    ///
    /// If the `WgpuRenderState` does not have a valid `FrameState`, the function
    /// exits early without performing any actions.
    fn apply_single(&mut self, wgpu_render: &mut WgpuRenderState, background_color: &BackgroundColorComponent) {
        let Some(state) = wgpu_render.frame_state_mut() else {
            return;
        };
        let _render_pass = state
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Background Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &state.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(background_color.as_wgpu_color()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
    }
}
