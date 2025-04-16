use std::convert::Infallible;
use tracing::{debug, trace};
use wgpu_resource_manager::{wgpu_label, DeviceAndQueue, DeviceId, SurfaceId, TextureWithView};
use crate::engine::{ApplyWgpuInitShaderSystem, ApplyWgpuInitTextureSystem, CreateSystem, EntityId, PhaseEvents, SystemFactory, SystemWgpuReinitPhaseEvents, TextureSourceComponent, WgpuInitShaderSystem, WgpuInitTextureSystem, WgpuShaderComponent, WgpuTextureComponent};
use crate::engine::components::TextureSource;
use crate::engine::phases::render::WgpuRenderState;

#[derive(Debug, Default)]
pub struct WgpuInitTextureSystemData {
    device_id: DeviceId,
    surface_id: SurfaceId
}

impl CreateSystem<WgpuInitTextureSystem> for SystemFactory {
    fn create(&self) -> WgpuInitTextureSystem {
        WgpuInitTextureSystem(WgpuInitTextureSystemData::default())
    }
}

impl ApplyWgpuInitTextureSystem for WgpuInitTextureSystem {
    type Error = Infallible;

    fn is_ready(&self, gpu: &mut WgpuRenderState) -> bool {
        gpu.is_ready()
    }

    fn on_end_phase(&mut self, wgpu_render: &mut WgpuRenderState) -> Result<(), Self::Error> {
        // TODO: Collect the created resources into a shared state for later access, e.g. runtime material lookups
        Ok(())
    }

    fn apply_many(&mut self, gpu: &mut WgpuRenderState, _entities: &[EntityId], sources: &[TextureSourceComponent], textures: &mut [WgpuTextureComponent]) {
        let current_surface_id = gpu.surface_id();
        let _surface_changed = current_surface_id != self.surface_id;
        self.surface_id = current_surface_id;

        let Ok(device) = gpu.device() else {
            return;
        };

        let current_device_id = device.id();
        let device_changed = current_device_id != self.device_id;
        self.device_id = current_device_id;

        // TODO: Check surface-dependent textures
        for ((entity_id, source), texture) in _entities.iter().zip(sources).zip(textures.iter_mut()) {
            let texture_missing = texture.twv.is_none();
            match source.source {
                TextureSource::FallbackNormals => {
                    if device_changed || texture_missing {
                        create_fallback_normals(entity_id, device, texture)
                    }
                },
            };
        }
    }
}

fn create_fallback_normals(entity_id: &EntityId, device: &DeviceAndQueue, out: &mut WgpuTextureComponent) {
    let label = wgpu_label!(format!("fallback_normal_map ({entity_id})"));
    debug_log(entity_id, device, &label);

    // Flat normal: (0.5, 0.5, 1.0, 1.0) encoded as [128, 128, 255, 255]
    let normal_pixel = [128u8, 128u8, 255u8, 255u8];

    let size = wgpu::Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 1,
    };

    let texture = device.device().create_texture(&wgpu::TextureDescriptor {
        label: Some(&label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        // Use linear format for normal maps
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    device.queue().write_texture(
        wgpu::TexelCopyTextureInfo {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &normal_pixel,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * 1),
            rows_per_image: Some(1),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    out.twv = Some(TextureWithView::new(texture, view));
}

fn debug_log(entity_id: &EntityId, device: &DeviceAndQueue, label: &str) {
    if tracing::enabled!(tracing::Level::TRACE) {
        debug!( // sic!
            %entity_id,
            %label,
            device_id = %device.id(),
            "Initializing texture"
        );
    } else {
        debug!(
            %entity_id,
            device_id = %device.id(),
            "Initializing texture"
        );
    }
}
