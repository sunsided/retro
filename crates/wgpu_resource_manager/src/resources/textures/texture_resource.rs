use std::error::Error;
use crate::resources::{GetResourceError, RecreateReason, Resources};
use std::fmt::Debug;
use std::marker::PhantomData;
use wgpu::{Texture, TextureView};
use crate::{GpuContext, Resource, TextureResourceId};
use crate::resource::ResourceData;
use crate::resources::macros::define_resource_boilerplate;
use crate::surface_info::SurfaceInfo;

#[allow(dead_code)]
pub trait TextureResourceData: Debug {
    fn texture(&self) -> &Texture;
    fn view(&self) -> &TextureView;
}

define_resource_boilerplate!(
    texture,
    TextureResourceData,
    BoxedTextureResourceData,
    TextureResourceEntry,
    TextureResourceFactory,
    textures,
    register_texture,
    get_texture_data,
    retain_textures,
    recreate_textures,
    TextureResourceId
);
