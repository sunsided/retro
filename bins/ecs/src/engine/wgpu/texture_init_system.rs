use crate::engine::components::Extent3d;
use crate::engine::phases::render::WgpuRenderState;
use crate::engine::{
    ApplyWgpuInitShaderSystem, ApplyWgpuInitTextureSystem, CreateSystem, EntityId, PhaseEvents,
    SystemFactory, SystemWgpuReinitPhaseEvents, TextureResourcePath, TextureSourceComponent,
    TextureSourceType, WgpuInitShaderSystem, WgpuInitTextureSystem, WgpuShaderComponent,
    WgpuTextureComponent,
};
use crate::resources;
use crate::wgpu::texture::{TextureError, TextureType};
use image::{GenericImageView, RgbaImage};
use std::borrow::Borrow;
use std::convert::Infallible;
use std::ops::Deref;
use tracing::{debug, error, trace};
use wgpu::{Queue, Texture};
use wgpu_resource_manager::{wgpu_label, DeviceAndQueue, DeviceId, SurfaceId, TextureWithView};

#[derive(Debug, Default)]
pub struct WgpuInitTextureSystemData {
    device_id: DeviceId,
    surface_id: SurfaceId,
}

impl CreateSystem<WgpuInitTextureSystem> for SystemFactory {
    fn create(&self) -> WgpuInitTextureSystem {
        WgpuInitTextureSystem(WgpuInitTextureSystemData::default())
    }
}

impl ApplyWgpuInitTextureSystem for WgpuInitTextureSystem {
    type Error = Infallible;

    fn is_ready(&self, gpu: &WgpuRenderState) -> bool {
        gpu.is_ready()
    }

    // TODO: Updating material databases could work multiple ways:
    //      - as a postflight: This will take a full scan of all resources even if they are unchanged
    //      - using custom commands: This will only be visible to the following phases, not to systems in this phase
    //      - using manipulation of this system's state and a phase-end hook: This hook is system-specific and executed before follow-up systems run (unless they are independent)

    fn preflight(
        &mut self,
        wgpu_render: &WgpuRenderState,
        entities: &[EntityId],
        texture_sources: &[TextureSourceComponent],
        wgpu_textures: &[WgpuTextureComponent],
    ) {
    }

    fn apply_many(
        &mut self,
        gpu: &mut WgpuRenderState,
        _entities: &[EntityId],
        sources: &[TextureSourceComponent],
        textures: &mut [WgpuTextureComponent],
    ) {
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
        for ((entity_id, source), texture) in _entities.iter().zip(sources).zip(textures.iter_mut())
        {
            let texture_missing = texture.twv.is_none();
            match &source.source {
                TextureSourceType::FallbackNormals => {
                    if device_changed || texture_missing {
                        create_fallback_normals(entity_id, device, texture)
                    }
                }
                TextureSourceType::Resource(path) => {
                    create_from_resource(entity_id, device, path, texture)
                }
            };
        }
    }
}

fn create_from_resource(
    entity_id: &EntityId,
    device: &DeviceAndQueue,
    path: &TextureResourcePath,
    out: &mut WgpuTextureComponent,
) {
    let label = wgpu_label!(format!(
        "resource {path:?} ({entity_id})",
        path = path.as_ref()
    ));
    let img = match resources::load_image(path).map_err(TextureError::LoadFromFile) {
        Ok(img) => img,
        Err(error) => {
            error!(?error, %entity_id, path = ?path.as_ref(), "Failed to load texture from resource");
            // TODO: Replace with statically-known fallback texture (from memory)
            return;
        }
    };

    debug_log(entity_id, device, &label);

    let data = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture = device.device().create_texture(&wgpu::TextureDescriptor {
        label: Some(&label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: TextureType::AlbedoRgbaSrgb.into(),
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        // This is the same as with the SurfaceConfig. It specifies what texture formats can be used to
        // create TextureViews for this texture. The base texture format (Rgba8UnormSrgb in this case) is
        // always supported. Note that using a different texture format is not supported on the WebGL2 backend.
        view_formats: &[],
    });

    upload_texture_data(device.queue(), &texture, size, &data);

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    out.twv = Some(TextureWithView::new(texture, view));
}

fn create_fallback_normals(
    entity_id: &EntityId,
    device: &DeviceAndQueue,
    out: &mut WgpuTextureComponent,
) {
    let label = wgpu_label!(format!("fallback_normal_map ({entity_id})"));
    debug_log(entity_id, device, &label);

    // Flat normal: (0.5, 0.5, 1.0, 1.0) encoded as [128, 128, 255, 255]
    let data = [128u8, 128u8, 255u8, 255u8];

    let size = Extent3d {
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
        format: wgpu::TextureFormat::Rgba8Unorm, // Use linear format (non-sRGB) for normal maps
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    upload_texture_data(device.queue(), &texture, size, &data);

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    out.twv = Some(TextureWithView::new(texture, view));
}

fn upload_texture_data(queue: &Queue, texture: &Texture, size: Extent3d, data: &[u8]) {
    queue.write_texture(
        copy_all(&texture),
        data.into(),
        full_rgba_layout(&size),
        size,
    );
}

fn copy_all(texture: &Texture) -> wgpu::TexelCopyTextureInfo {
    wgpu::TexelCopyTextureInfo {
        aspect: wgpu::TextureAspect::All,
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
    }
}

fn full_rgba_layout(size: &Extent3d) -> wgpu::TexelCopyBufferLayout {
    wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * size.width),
        rows_per_image: Some(size.height),
    }
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
