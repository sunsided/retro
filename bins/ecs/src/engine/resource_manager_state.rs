use crate::engine::WgpuShaderData;
use fxhash::FxHashMap;
use sillyecs::EntityId;
use wgpu::{BindGroup, ShaderModule};
use wgpu_resource_manager::TextureWithView;

#[derive(Debug, Default)]
pub struct ResourceManagerState {
    pub shaders: FxHashMap<EntityId, ShaderModule>,
    pub textures: FxHashMap<EntityId, TextureWithView>,
    pub bind_groups: FxHashMap<EntityId, BindGroup>,
}
