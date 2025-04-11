use crate::wgpu::model::{Mesh, Model};
use std::ops::Range;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    pub color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding2: u32,
}

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            _padding: 0,
            color,
            _padding2: 0,
        }
    }
}

pub trait DrawLight<'model> {
    #[allow(dead_code)]
    fn draw_light_mesh(
        &mut self,
        mesh: &'model Mesh,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    );

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'model Mesh,
        instances: Range<u32>,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    );

    fn draw_light_model(
        &mut self,
        model: &'model Model,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    );
    fn draw_light_model_instanced(
        &mut self,
        model: &'model Model,
        instances: Range<u32>,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    );
}

impl<'pass, 'model> DrawLight<'model> for wgpu::RenderPass<'pass>
where
    'model: 'pass,
{
    fn draw_light_mesh(
        &mut self,
        mesh: &'model Mesh,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.draw_light_mesh_instanced(mesh, 0..1, camera_bind_group, light_bind_group);
    }

    fn draw_light_mesh_instanced(
        &mut self,
        mesh: &'model Mesh,
        instances: Range<u32>,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.set_bind_group(1, light_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_light_model(
        &mut self,
        model: &'model Model,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.draw_light_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
    }
    fn draw_light_model_instanced(
        &mut self,
        model: &'model Model,
        instances: Range<u32>,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_light_mesh_instanced(
                mesh,
                instances.clone(),
                camera_bind_group,
                light_bind_group,
            );
        }
    }
}
