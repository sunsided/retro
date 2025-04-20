use crate::engine::WgpuShaderData;
use dashmap::DashMap;
use fxhash::{FxHashMap, FxHasher};
use sillyecs::EntityId;
use std::hash::BuildHasherDefault;
use wgpu::ShaderModule;
use wgpu_resource_manager::TextureWithView;

// A zero‑sized hasher builder for FxHasher:
type FxBuildHasher = BuildHasherDefault<FxHasher>;

#[derive(Debug, Default)]
pub struct ResourceManagerState {
    pub shaders: DashMap<EntityId, ShaderModule, FxBuildHasher>,
    pub textures: DashMap<EntityId, TextureWithView, FxBuildHasher>,
}
