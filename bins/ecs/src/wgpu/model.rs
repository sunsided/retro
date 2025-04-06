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

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh);
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: Range<u32>,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh) {
        self.draw_mesh_instanced(mesh, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: Range<u32>,
    ){
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
