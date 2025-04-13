use wgpu::{vertex_attr_array, VertexAttribute};

#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TileInstance {
    pub transform: [[f32; 4]; 4],
    pub tile_index: u32,
    pub decoration_index: u32,
    pub entity_id: u32,
}

impl TileVertex {
    #[allow(unused_qualifications)]
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const LAYOUT: [VertexAttribute; 2] = vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2,
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &LAYOUT,
        }
    }
}

impl TileInstance {
    #[allow(unused_qualifications)]
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const LAYOUT: [VertexAttribute; 7] = vertex_attr_array![
            2 => Float32x4,
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
            6 => Uint32,
            7 => Uint32,
            8 => Uint32,
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &LAYOUT,
        }
    }
}
