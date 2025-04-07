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

use crate::wgpu::gpu_hardware::RenderPipelineSelector;
use std::convert::Infallible;
use tracing::warn;
use wgpu::{BindGroup, IndexFormat};
use crate::engine::{ApplyRenderBackgroundSystem, BackgroundColorComponent, RenderBackgroundSystem};
use crate::engine::phases::render::WgpuRenderState;
use crate::wgpu::model::DrawModel;
use crate::wgpu::light::DrawLight;
use crate::wgpu::texture::Texture;

/// Represents a background color with red, green, blue, and alpha components,
/// used as part of the [`BackgroundColorComponent`].
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

#[derive(Debug, Default)]
pub struct RenderBackgroundSystemData {
    pub selector: RenderPipelineSelector,
    pub texture: Option<BindGroup>
}

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
        let Some(device) = wgpu_render.hardware.device() else {
            warn!("Skipping background rendering because no device is set.");
            return;
        };

        let Some(frame_state) = &mut wgpu_render.frame_state else {
            warn!("Skipping background rendering because frame state is not set.");
            return;
        };

        let mut render_pass = frame_state
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Background Render Pass"),
                // NOTE: The color_attachments field is a "sparse" array. This allows you to use a
                //       pipeline that expects multiple render targets and only supplies the ones you care about.
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &frame_state.view,
                        // The resolve_target is the texture that will receive the resolved output.
                        // This will be the same as view unless multisampling is enabled.
                        // We don't need to specify this, so we leave it as None.
                        resolve_target: None,
                        ops: wgpu::Operations {
                            // The `load` field tells wgpu how to handle colors stored from the
                            // previous frame. Currently, we are clearing the screen with a color.
                            load: wgpu::LoadOp::Clear(background_color.as_wgpu_color()),
                            // The `store` field tells wgpu whether we want to store the rendered
                            // results to the Texture behind our TextureView (in this case, it's
                            // the SurfaceTexture). We use StoreOp::Store as we do want to store
                            // our render results.
                            store: wgpu::StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &device.depths().screen.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });


        // Enable the instance buffer.
        render_pass.set_vertex_buffer(1, device.buffers().instances.slice(..));
        let num_instances = device.buffers().instances.len() as u32;

        // Render the light.
        render_pass.set_pipeline(&device.pipelines().light);
        render_pass.draw_light_model(
            &device.models().cube,
            &device.bind_groups().camera,
            &device.bind_groups().light_vb,
        );

        // Render the normal thing.
        render_pass.set_pipeline(device.pipelines().render_pipeline(self.selector));
        render_pass.draw_model_instanced(&device.models().cube, 0..num_instances, &device.bind_groups().camera, &device.bind_groups().light_vb);
    }
}
