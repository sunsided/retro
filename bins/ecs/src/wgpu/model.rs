use std::ops::Range;
use crate::wgpu::texture::Texture;

/// Vertex trait.
///
/// We could have multiple vertex types (model, UI, instance data, etc.).
/// Making `Vertex` a trait will allow us to abstract out the `VertexBufferLayout` creation code
/// to make creating `RenderPipeline`s simpler.
pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

const MODEL_VERTEX_BUFFER_ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
        0 => Float32x3, // vertex shader's @position(0)
        1 => Float32x2,  // vertex shader's @position(1)
        2 => Float32x3  // vertex shader's @position(2)
    ];

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &MODEL_VERTEX_BUFFER_ATTRIBUTES,
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: Box<str>,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: Box<str>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub trait DrawModel<'mesh> {
    fn draw_mesh(&mut self, mesh: &'mesh Mesh, material: &'mesh Material, camera_bind_group: &'mesh wgpu::BindGroup);

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'mesh Mesh,
        material: &'mesh Material,
        instances: Range<u32>,
        camera_bind_group: &'mesh wgpu::BindGroup,
    );
}

impl<'pass, 'mesh> DrawModel<'mesh> for wgpu::RenderPass<'pass>
where
    'mesh: 'pass,
{
    fn draw_mesh(&mut self, mesh: &'mesh Mesh, material: &'mesh Material, camera_bind_group: &'mesh wgpu::BindGroup) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'mesh Mesh,
        material: &'mesh Material,
        instances: Range<u32>,
        camera_bind_group: &'mesh wgpu::BindGroup,
    ){
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
