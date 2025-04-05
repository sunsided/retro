#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }

    // Returns vertices for a rectangle given x, y, width, height.
    fn rect_vertices(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
        vec![
            Vertex {
                position: [x, y],
            },
            Vertex {
                position: [x + width, y],
            },
            Vertex {
                position: [x, y + height],
            },
            Vertex {
                position: [x + width, y],
            },
            Vertex {
                position: [x + width, y + height],
            },
            Vertex {
                position: [x, y + height],
            },
        ]
    }
}
