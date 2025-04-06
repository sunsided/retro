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
