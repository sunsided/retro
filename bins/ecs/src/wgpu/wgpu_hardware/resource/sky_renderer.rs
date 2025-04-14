use crate::wgpu::wgpu_hardware::resources::bind_group_layouts::BindGroupLayouts;
use wgpu::Device;

pub struct SkyRenderer {}

impl SkyRenderer {
    pub fn create(_device: &Device, _bind_group_layouts: &BindGroupLayouts) -> Self {
        todo!()
    }
}
