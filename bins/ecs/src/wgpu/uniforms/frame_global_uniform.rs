//! Provides the [`FrameGlobalUniform`], a uniform that changes only once per frame.

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameGlobalUniform {
    pub time_of_day: f32,     // e.g. 13.5 = 1:30 PM
    pub star_visibility: f32, // 0.0 (day) to 1.0 (night)
    pub _pad0: [f32; 2],

    pub sun_direction: [f32; 3], // normalized world-space direction to the sun
    pub sun_visibility: f32,     // 0.0 (sun below horizon) to 1.0 (fully visible)

    pub sun_color: [f32; 3], // RGB tint of sun light (reddish at horizon)
    pub _pad1: f32,

    pub sky_color: [f32; 3], // RGB sky dome base color (interpolated day/night)
    pub _pad2: f32,

    pub ambient_light: [f32; 3], // RGB ambient lighting (for scene)
    pub _pad3: f32,

    pub _pad4: [f32; 2],
}

impl FrameGlobalUniform {
    pub fn new(time_of_day: f32) -> Self {
        Self {
            time_of_day,
            sun_direction: [0.0, 1.0, 0.0], // overhead sun
            sun_visibility: 1.0,
            sun_color: [1.2, 1.05, 0.85],    // warm midday sun
            sky_color: [0.4, 0.75, 1.0],     // bright daytime sky
            ambient_light: [0.2, 0.25, 0.3], // soft daylight ambient
            star_visibility: 0.0,
            _pad0: Default::default(),
            _pad1: Default::default(),
            _pad2: Default::default(),
            _pad3: Default::default(),
            _pad4: Default::default(),
        }
    }
}
