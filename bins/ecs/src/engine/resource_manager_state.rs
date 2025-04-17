use crate::engine::{EntityId, WgpuShaderData};
use fxhash::FxHashMap;
use wgpu::ShaderModule;
use wgpu_resource_manager::TextureWithView;

#[derive(Debug, Default)]
pub struct ResourceManagerState {
    pub shaders: FxHashMap<EntityId, ShaderModule>,
    pub textures: FxHashMap<EntityId, TextureWithView>,
}
