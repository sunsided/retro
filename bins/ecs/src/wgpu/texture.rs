//! # Texture Module
//!
//! This module provides utilities to handle GPU textures using the `wgpu` library.
//! It includes a `Texture` struct that encapsulates a GPU texture and its corresponding view,
//! along with methods to create textures from in-memory data or image files.
//!
//! ## Overview
//!
//! Textures are one of the primary assets used in graphics programming. They store data such as
//! color information and can be sampled in shaders for various purposes, including rendering
//! meshes or UI elements. This module simplifies the process of creating and managing textures
//! on the GPU.
//!
//! ## Features
//!
//! - Load textures from raw byte arrays or from decoded image data.
//! - GPU memory handling for textures using `wgpu` with basic options, such as 2D dimensions
//!   and a `Rgba8UnormSrgb` format.
//! - Bind textures to shaders for rendering purposes via the `wgpu::TextureView`.
//!
//! ## Error Handling
//!
//! The `TextureError` enum is used for handling errors when working with textures. It currently
//! handles errors related to loading images from memory.
//!
//! ## See Also
//!
//! - [`image`](https://docs.rs/image) crate: Used for loading and decoding image files.
//! - [`wgpu`](https://docs.rs/wgpu): A modern graphics API for GPU programming.

use std::path::Path;
use image::{GenericImageView, ImageError};
use wgpu::{Device, Queue, TextureFormat};
use crate::resources;
use crate::wgpu::wgpu_hardware::DeviceAndQueue;

/// A struct representing a GPU texture along with its corresponding view.
///
/// This structure encapsulates a `wgpu::Texture` object used for storing image
/// data on the GPU, and a `wgpu::TextureView` which provides a view into the texture.
///
/// The `Texture` struct is primarily used in rendering operations that require sampled
/// textures (e.g., applying textures to models or rendering UI elements).
///
/// ## See also
///
/// - [`TextureSampler`](crate::wgpu::texture_sampler::TextureSampler)
#[derive(Debug)]
pub struct Texture {
    /// The GPU texture resource that stores the image data.
    #[allow(unused)]
    pub texture: wgpu::Texture,
    /// A view into the texture, used for binding the texture as a resource in shaders.
    pub view: wgpu::TextureView,
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum TextureError {
    #[error("Failed to load image from bytes: {0}")]
    LoadFromBytes(ImageError),
    #[error("Failed to load image from file: {0}")]
    LoadFromFile(ImageError),
}

impl Texture {
    pub const VGA_WIDTH: u32 = 320;
    pub const VGA_HEIGHT: u32 = 200;
    #[allow(dead_code)]
    pub const VGA_BYTES_PER_PIXEL: u32 = 4; // 32 bpp (RGBA)
    #[allow(dead_code)]
    pub const VGA_BUFFER_SIZE: usize = (Self::VGA_WIDTH * Self::VGA_HEIGHT * Self::VGA_BYTES_PER_PIXEL) as usize;

    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    /// Creates a depth texture for use in rendering pipelines.
    ///
    /// This function generates a GPU texture configured for depth buffering,
    /// which is useful for managing object depth in 3D rendering. The resulting
    /// texture can be used as both a render attachment and a shader resource.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the `wgpu::Device` used to create the texture.
    /// - `config`: A reference to the `wgpu::SurfaceConfiguration` that defines the
    ///   dimensions of the texture.
    /// - `label`: A descriptive label for the texture, used for debugging purposes.
    ///
    /// # Returns
    ///
    /// Returns a `Texture` object containing the created depth texture and its
    /// corresponding view, ready for use in rendering.
    ///
    /// # See also
    /// - [`TextureSampler::depth_sampler`](crate::wgpu::texture_sampler::TextureSampler::depth_sampler)
    pub fn depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    /// Creates a texture from a raw byte array representing an image.
    ///
    /// This function takes a byte slice containing the raw image data, decodes it into an image object,
    /// and then uploads it to the GPU as a texture. The texture can be used in rendering pipelines,
    /// such as sampling in shaders.
    ///
    /// # Parameters
    ///
    /// - `bytes`: A byte array containing the raw image data to be decoded and uploaded.
    /// - `device`: A reference to the `wgpu::Device`, used to create GPU resources.
    /// - `queue`: A reference to the `wgpu::Queue`, used to upload texture data to the GPU.
    /// - `ty`: The type of texture to load.
    /// - `label`: A descriptive label for the texture, used for debugging purposes.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created `Texture` object if successful, or a
    /// `TextureError` if decoding the image from the byte array fails.
    ///
    /// # Errors
    ///
    /// This function will return a `TextureError::LoadFromBytes` if the image decoding fails.
    #[allow(dead_code)]
    pub fn from_bytes_with(
        bytes: &[u8],
        device: &Device,
        queue: &Queue,
        ty: TextureType,
        label: &str,
    ) -> Result<Self, TextureError> {
        let img = image::load_from_memory(bytes).map_err(TextureError::LoadFromBytes)?;
        Ok(Self::from_image_with(device, queue, &img, ty, Some(label)))
    }

    /// Creates a texture from a decoded image.
    ///
    /// This function converts a `DynamicImage` into a format suitable for use as a GPU texture
    /// and uploads it to the GPU. The resulting texture can be sampled in rendering operations,
    /// such as shaders, by using the texture's view.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the `wgpu::Device`, used to create the GPU texture resource.
    /// - `queue`: A reference to the `wgpu::Queue`, used to upload texture data to the GPU.
    /// - `layout`: A reference to the `wgpu::BindGroupLayout`.
    /// - `sampler`: A reference to the `wgpu::Sampler`.
    /// - `img`: A reference to the `DynamicImage` object to be converted into a GPU texture.
    /// - `label`: An optional label for the texture, useful for debugging purposes.
    ///
    /// # Returns
    ///
    /// Returns a newly created [`Texture`] object. This function does not fail since it assumes
    /// that the provided `DynamicImage` is valid and can be converted into a GPU-compatible format.
    pub fn from_image_with(
        device: &Device,
        queue: &Queue,
        img: &image::DynamicImage,
        ty: TextureType,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB, so we need to reflect that here.
                format: ty.into(),
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                // This is the same as with the SurfaceConfig. It
                // specifies what texture formats can be used to
                // create TextureViews for this texture. The base
                // texture format (Rgba8UnormSrgb in this case) is
                // always supported. Note that using a different
                // texture format is not supported on the WebGL2
                // backend.
                view_formats: &[],
            }
        );

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }

    /// Creates a fallback normal map texture for cases where a normal map is missing.
    ///
    /// This function generates a small 1x1 GPU texture with a flat normal value
    /// `(0.5, 0.5, 1.0, 1.0)` represented as `[128, 128, 255, 255]`. It is often used
    /// when no normal map texture is provided for a material, ensuring consistency
    /// in rendering pipelines.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the `wgpu::Device`, used to create the GPU texture resource.
    /// - `queue`: A reference to the `wgpu::Queue`, used to upload texture data to the GPU.
    ///
    /// # Returns
    ///
    /// Returns a `Texture` object containing the fallback normal map and its view, ready for use
    /// in rendering and sampling operations.
    pub fn fallback_normal_map(
        device: &Device,
        queue: &Queue,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("fallback_normal_map"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Use linear format for normal maps
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        // Flat normal: (0.5, 0.5, 1.0, 1.0) encoded as [128, 128, 255, 255]
        let normal_pixel = [128u8, 128u8, 255u8, 255u8];

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &normal_pixel,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * 1),
                rows_per_image: Some(1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }
}

#[cfg(any(feature = "jpeg", feature = "png", feature = "webp"))]
impl Texture {
    /// Loads a texture from an image file and uploads it to the GPU using the provided `WgpuState`.
    ///
    /// This function simplifies texture creation by utilizing a `WgpuState` object, which manages the
    /// `wgpu::Device` and `wgpu::Queue`. It reads an image file from the specified path, decodes it into
    /// a `DynamicImage`, and creates a GPU texture.
    ///
    /// # Parameters
    ///
    /// - `path`: The file path to the image file to be loaded.
    /// - `state`: A reference to the `WgpuState` object, which provides access to the `wgpu::Device` and
    ///   `wgpu::Queue` required for texture creation and data uploading.
    /// - `label`: An optional label added to the texture for easier debugging.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the created `Texture` object on success, or a `TextureError` if the
    /// image could not be loaded or decoded.
    ///
    /// # Errors
    ///
    /// This function will return a `TextureError::LoadFromFile` if the image file could not be loaded or
    /// decoded properly.
    #[allow(dead_code)]
    pub fn from_file<P>(
        path: P,
        state: &DeviceAndQueue,
        ty: TextureType,
        label: Option<&str>,
    ) -> Result<Self, TextureError>
    where
        P: AsRef<Path>,
    {
        Self::from_file_with(path, state.device(), state.queue(), ty, label)
    }

    /// Loads a texture from an image file and uploads it to the GPU using the provided `WgpuState`.
    ///
    /// This function simplifies texture creation by utilizing a `WgpuState` object, which manages the
    /// `wgpu::Device` and `wgpu::Queue`. It reads an image file from the specified path, decodes it into
    /// a `DynamicImage`, and creates a GPU texture.
    ///
    /// # Parameters
    ///
    /// - `path`: The file path to the image file to be loaded.
    /// - `device`: The `wgpu::Device` required for texture creation and data uploading.
    /// - `queue`: The `wgpu::Queue` required for texture creation and data uploading.
    /// - `label`: An optional label added to the texture for easier debugging.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the created `Texture` object on success, or a `TextureError` if the
    /// image could not be loaded or decoded.
    ///
    /// # Errors
    ///
    /// This function will return a `TextureError::LoadFromFile` if the image file could not be loaded or
    /// decoded properly.
    pub fn from_file_with<P>(
        path: P,
        device: &Device,
        queue: &Queue,
        ty: TextureType,
        label: Option<&str>,
    ) -> Result<Self, TextureError>
    where
        P: AsRef<Path>,
    {
        let img = resources::load_image(path).map_err(TextureError::LoadFromFile)?;
        Ok(Self::from_image_with(device, queue, &img, ty, label))
    }
}

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub enum TextureType {
    #[default]
    Diffuse,
    Normal,
}

impl TextureType {
    pub fn to_format(&self) -> TextureFormat {
        match self {
            TextureType::Diffuse => TextureFormat::Rgba8UnormSrgb,
            TextureType::Normal => TextureFormat::Rgba8Unorm,
        }
    }
}

impl From<TextureType> for TextureFormat {
    fn from(value: TextureType) -> Self {
        value.to_format()
    }
}
