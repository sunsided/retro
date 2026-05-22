use crate::engine::phases::render::{FrameState, WgpuRenderState};
use crate::engine::{
    ApplyRenderBackgroundSystem, BackgroundColorComponent, CreateSystem, RenderBackgroundSystem,
    SystemFactory,
};
use crate::wgpu::model::DrawModel;
use crate::wgpu::texture::Texture;
use crate::wgpu::wgpu_hardware::{DeviceAndQueue, RenderPipelineSelector};
use std::convert::Infallible;
use tracing::warn;
use wgpu::{BindGroup, IndexFormat, RenderPass, TextureView};

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
    /// Dims the background color by multiplying each color component (`r`, `g`, `b`)
    /// with the given factor, while keeping alpha (`a`) unchanged.
    ///
    /// # Parameters
    /// - `factor`: A `f32` value between [0.0, 1.0], where 0.0 results in black,
    ///   and 1.0 keeps the color unchanged. Values outside this range will be clamped.
    ///
    /// # Returns
    /// A new `BackgroundColorData` instance with the dimmed colors.
    pub fn dim(&self, factor: f32) -> Self {
        let clamped_factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.r * clamped_factor,
            g: self.g * clamped_factor,
            b: self.b * clamped_factor,
            a: self.a, // Keep alpha unchanged
        }
    }
}

impl BackgroundColorComponent {
    /// Dims the background color by multiplying each color component (`r`, `g`, `b`)
    /// with the given factor, while keeping alpha (`a`) unchanged.
    ///
    /// # Parameters
    /// - `factor`: A `f32` value between [0.0, 1.0], where 0.0 results in black,
    ///   and 1.0 keeps the color unchanged. Values outside this range will be clamped.
    ///
    /// # Returns
    /// A new `BackgroundColorData` instance with the dimmed colors.
    #[inline]
    pub fn dim(&self, factor: f32) -> Self {
        Self(self.0.dim(factor))
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
pub struct RenderBackgroundSystemData;

impl CreateSystem<RenderBackgroundSystem> for SystemFactory {
    fn create(&self) -> RenderBackgroundSystem {
        RenderBackgroundSystem(RenderBackgroundSystemData)
    }
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
    ///   contains the current frame's state, including the `CommandEncoder`
    ///   and `TextureView`.
    /// - `background_color`: A reference to the `BackgroundColorComponent`
    ///   that specifies the clear color to apply.
    ///
    /// If the `WgpuRenderState` does not have a valid `FrameState`, the function
    /// exits early without performing any actions.
    fn apply_single(
        &mut self,
        gpu: &mut WgpuRenderState,
        background_color: &BackgroundColorComponent,
    ) {
        let Ok((_device, frame_state)) = gpu.for_rendering() else {
            warn!("Skipping background rendering because hardware or frame is not ready.");
            return;
        };

        // Clear the background
        let _ = frame_state.passes.clear_background_pass(background_color);
    }
}
