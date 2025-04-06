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

use image::{GenericImageView, ImageError};

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
pub struct Texture {
    /// The GPU texture resource that stores the image data.
    #[allow(unused)]
    pub texture: wgpu::Texture,

    /// A view into the texture, used for binding the texture as a resource in shaders.
    pub view: wgpu::TextureView,
}

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
    #[error("Failed to load image from bytes: {0}")]
    LoadFromBytes(ImageError),
    #[error("Failed to load image from file: {0}")]
    LoadFromFile(ImageError),
}

impl Texture {
    /// Creates a texture from a raw byte array representing an image.
    ///
    /// This function takes a byte slice containing the raw image data, decodes it into an image object,
    /// and then uploads it to the GPU as a texture. The texture can be used in rendering pipelines,
    /// such as sampling in shaders.
    ///
    /// # Parameters
    ///
    /// - `device`: A reference to the `wgpu::Device`, used to create GPU resources.
    /// - `queue`: A reference to the `wgpu::Queue`, used to upload texture data to the GPU.
    /// - `bytes`: A byte array containing the raw image data to be decoded and uploaded.
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
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self, TextureError> {
        let img = image::load_from_memory(bytes).map_err(TextureError::LoadFromBytes)?;
        Ok(Self::from_image(device, queue, &img, Some(label)))
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
    /// - `img`: A reference to the `DynamicImage` object to be converted into a GPU texture.
    /// - `label`: An optional label for the texture, useful for debugging purposes.
    ///
    /// # Returns
    ///
    /// Returns a newly created [`Texture`] object. This function does not fail since it assumes
    /// that the provided `DynamicImage` is valid and can be converted into a GPU-compatible format.
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
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
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
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
}

#[cfg(any(feature = "jpeg", feature = "png", feature = "webp"))]
impl Texture {
    /// Loads a texture from an image file and uploads it to the GPU.
    ///
    /// This function reads an image file from the specified path, decodes it into a `DynamicImage`,
    /// and creates a `Texture` object on the GPU using the `from_image` method.
    ///
    /// # Parameters
    ///
    /// - `path`: The file path to the image file to be loaded.
    /// - `device`: A reference to the `wgpu::Device`, used to create GPU resources.
    /// - `queue`: A reference to the `wgpu::Queue`, used to upload texture data to the GPU.
    /// - `label`: An optional label for the texture, useful for debugging purposes.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the created `Texture` object if successful, or a
    /// `TextureError` if loading the image from the file fails.
    ///
    /// # Errors
    ///
    /// This function will return a `TextureError::LoadFromBytes` if loading or decoding the image fails.
    pub fn from_file<P>(
        path: P,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Result<Self, TextureError>
    where
        P: AsRef<std::path::Path>,
    {
        let img = image::open(path).map_err(TextureError::LoadFromFile)?;
        Ok(Self::from_image(device, queue, &img, label))
    }
}
