#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    /// The vertex data for a pentagon in counter-clockwise order.
    pub const PENTAGON_VERTICES: [Vertex; 5] = [
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
    ];

    /// The vertex indices of the triangles forming the pentagon.
    /// Refer to [`PENTAGON_VERTICES`](Self::PENTAGON_VERTICES) above for the actual locations.
    #[rustfmt::skip]
    pub const PENTAGON_INDICES: [u16; 9] = [
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];

    // attributes: &[
    //     wgpu::VertexAttribute {
    //         offset: 0,
    //         shader_location: 0, // vertex shader's @position(0)
    //         format: wgpu::VertexFormat::Float32x3,
    //     },
    //     wgpu::VertexAttribute {
    //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
    //         shader_location: 1, // vertex shader's @position(1)
    //         format: wgpu::VertexFormat::Float32x3,
    //     }
    // ]
    const VERTEX_BUFFER_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // vertex shader's @position(0)
        1 => Float32x3  // vertex shader's @position(1)
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_BUFFER_ATTRIBUTES
        }
    }
}
