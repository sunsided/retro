use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{GetResourceError, GpuContext, RecreateReason, Resource, ResourceData, Resources, SurfaceInfo, TextureSamplerResourceId};
use crate::resources::macros::define_resource_boilerplate;

#[allow(dead_code)]
pub trait TextureSamplerResourceData: Debug {
    fn texture_sampler(&self) -> &wgpu::Sampler;
}

define_resource_boilerplate!(
    texture_sampler,
    TextureSamplerResourceData,
    BoxedTextureSamplerResourceData,
    TextureSamplerResourceEntry,
    ResourceFactory,
    texture_samplers,
    register_texture_sampler,
    get_texture_sampler_data,
    retain_texture_samplers,
    recreate_texture_samplers,
    TextureSamplerResourceId
);
