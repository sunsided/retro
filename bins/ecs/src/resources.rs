use std::io;
use std::io::{BufReader, Cursor, Read};
use tobj::LoadError;
use tracing::error;
use wgpu::util::DeviceExt;
use crate::wgpu::model::{Material, Mesh, Model, ModelVertex};
use crate::wgpu::texture::{Texture, TextureError};

pub fn load_string<P>(file_name: P) -> io::Result<String>
where
    P: AsRef<std::path::Path>,
{
    // TODO: Use proper path resolution.
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("resources")
        .join(file_name);
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn load_binary<P>(file_name: P) -> std::io::Result<Vec<u8>>
where
    P: AsRef<std::path::Path>,
{
    // TODO: Use proper path resolution.
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("resources")
        .join(file_name);
    let mut file = std::fs::File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn load_texture<P>(
    file_name: P,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<Texture, LoadTextureError>
where P: AsRef<std::path::Path>
{
    let file_name = file_name.as_ref();
    let label = file_name.to_string_lossy();
    let data = load_binary(file_name)?;
    let texture = Texture::from_bytes_with(&data, device, queue, &label)?;
    Ok(texture)
}

#[derive(Debug, thiserror::Error)]
pub enum LoadTextureError {
    #[error(transparent)]
    TextureError(#[from] TextureError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ModelLoadError {
    #[error("Failed to load model: {0}")]
    LoadModelError(tobj::LoadError),
    #[error("Failed to load material: {0}")]
    LoadMaterialError(tobj::LoadError),
    #[error("Failed to load texture: {0}")]
    LoadTextureError(#[from] LoadTextureError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("The material {0} does not have a texture assigned to it")]
    MaterialWithoutTexture(String)
}

pub async fn load_model<P>(
    file_name: P,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
) -> Result<Model, ModelLoadError>
where P: AsRef<std::path::Path>
{
    let file_name = file_name.as_ref();
    let obj_text = load_string(&file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        move |p| {
            let abs_p = file_name.parent().unwrap_or_else(|| std::path::Path::new(".")).join(p);
            let mat_text = load_string(&abs_p)
                .inspect_err(|err| error!(%err, path = ?abs_p, "Failed to load material"))
                .map_err(|_| LoadError::OpenFileFailed)?;
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    ).map_err(ModelLoadError::LoadModelError)?;

    let mut materials = Vec::new();
    for m in obj_materials.map_err(ModelLoadError::LoadMaterialError)? {
        // Load the diffuse map
        let diffuse_texture = m.diffuse_texture.ok_or_else(|| ModelLoadError::MaterialWithoutTexture(m.name.clone()))?;
        let diffuse_texture = file_name.parent().unwrap_or_else(|| std::path::Path::new(".")).join(diffuse_texture);
        let diffuse_texture = load_texture(&diffuse_texture, device, queue)
            .inspect_err(|err| error!(%err, path = ?diffuse_texture, "Failed to load diffuse texture"))?;

        // Load the normal map
        let normal_texture = m.normal_texture.ok_or_else(|| ModelLoadError::MaterialWithoutTexture(m.name.clone()))?;
        let normal_texture = file_name.parent().unwrap_or_else(|| std::path::Path::new(".")).join(normal_texture);
        let normal_texture = load_texture(&normal_texture, device, queue)
            .inspect_err(|err| error!(%err, path = ?normal_texture, "Failed to load diffuse texture"))?;

        materials.push(Material::new(
            &device,
            &m.name,
            diffuse_texture,
            normal_texture,
            layout,
            sampler
        ));
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    if m.mesh.normals.is_empty(){
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: [0.0, 0.0, 0.0],
                        }
                    }else{
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                        }
                    }
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            Mesh {
                name: file_name.to_string_lossy().to_string().into_boxed_str(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(Model { meshes, materials })
}
