use crate::wgpu::texture::Texture;
use std::ops::Range;

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
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}

const MODEL_VERTEX_BUFFER_ATTRIBUTES: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
    0 => Float32x3, // vertex shader's @position(0)
    1 => Float32x2, // vertex shader's @position(1)
    2 => Float32x3, // vertex shader's @position(2)
    3 => Float32x3, // vertex shader's @position(3)
    4 => Float32x3, // vertex shader's @position(4)
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

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Material {
    pub name: Box<str>,
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Mesh {
    pub name: Box<str>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Material {
    pub fn new(
        device: &wgpu::Device,
        name: &str,
        diffuse_texture: Texture,
        normal_texture: Texture,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                // Diffuse map
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
                // Normal map
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
            label: Some(name),
        });

        Self {
            name: String::from(name).into_boxed_str(),
            diffuse_texture,
            normal_texture, // NEW!
            bind_group,
        }
    }
}

pub trait DrawModel<'model> {
    #[inline]
    #[allow(dead_code)]
    fn draw_model(
        &mut self,
        model: &'model Model,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.draw_model_instanced(
            model,
            0..1,
            frame_global_bind_group,
            camera_bind_group,
            light_bind_group,
        );
    }

    fn draw_model_instanced(
        &mut self,
        model: &'model Model,
        instances: Range<u32>,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(
                mesh,
                material,
                instances.clone(),
                frame_global_bind_group,
                camera_bind_group,
                light_bind_group,
            );
        }
    }

    #[allow(dead_code)]
    fn draw_model_instanced_with_material(
        &mut self,
        model: &'model Model,
        material: &'model Material,
        instances: Range<u32>,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_mesh_instanced(
                mesh,
                material,
                instances.clone(),
                frame_global_bind_group,
                camera_bind_group,
                light_bind_group,
            );
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn draw_mesh(
        &mut self,
        mesh: &'model Mesh,
        material: &'model Material,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.draw_mesh_instanced(
            mesh,
            material,
            0..1,
            frame_global_bind_group,
            camera_bind_group,
            light_bind_group,
        );
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'model Mesh,
        material: &'model Material,
        instances: Range<u32>,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    );
}

impl<'pass, 'model> DrawModel<'model> for wgpu::RenderPass<'pass>
where
    'model: 'pass,
{
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'model Mesh,
        material: &'model Material,
        instances: Range<u32>,
        frame_global_bind_group: &'model wgpu::BindGroup,
        camera_bind_group: &'model wgpu::BindGroup,
        light_bind_group: &'model wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, frame_global_bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.set_bind_group(3, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
