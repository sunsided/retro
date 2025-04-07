use cgmath::Rotation3;
use crate::seconds::Seconds;
use crate::wgpu::gpu_hardware::DeviceAndQueue;
use crate::wgpu::light::LightUniform;

#[derive(Debug, Clone)]
pub struct DynamicLightData {
    pub uniform: LightUniform
}

impl Default for DynamicLightData {
    fn default() -> Self {
        Self {
            uniform: LightUniform::new(
                [2.0, 2.0, 2.0],
                [1.0, 1.0, 1.0]
            )
        }
    }
}

impl DynamicLightData {
    pub fn update_position(&mut self, delta_time: Seconds) {
        let rotation_speed = 60.0; // desired degrees per second
        let angle = delta_time.as_secs_f32() * rotation_speed;

        let old_position: cgmath::Vector3<_> = self.uniform.position.into();
        self.uniform.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(angle))
                * old_position)
                .into();
    }

    pub fn write_to_buffer(&self, gpu: &DeviceAndQueue) {
        gpu.queue().write_buffer(gpu.buffers().light_vb.as_ref(), 0, bytemuck::cast_slice(&[self.uniform]));
    }
}
