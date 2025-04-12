//! Provides the [`FrameGlobalUniform`], a uniform that changes only once per frame.

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameGlobalUniform {
    pub time_of_day: f32,
    // Padding to satisfy std140 alignment (4 floats total for the first vec4)
    pub _padding: [f32; 3],
}

impl FrameGlobalUniform {
    pub fn new(time_of_day: f32) -> Self {
        Self {
            time_of_day,
            _padding: [0.0; 3],
        }
    }
}
