use crate::wgpu::model::{Material, Mesh, Model, ModelVertex};
use crate::wgpu::texture::{Texture, TextureError, TextureType};
use image::DynamicImage;
use std::io;
use std::io::{BufReader, Cursor, Read};
use tobj::LoadError;
use tracing::error;
use wgpu::util::DeviceExt;

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

pub fn load_binary<P>(file_name: P) -> io::Result<Vec<u8>>
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

pub fn load_image<P>(file_name: P) -> image::ImageResult<DynamicImage>
where
    P: AsRef<std::path::Path>,
{
    // TODO: Use proper path resolution.
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("resources")
        .join(file_name);
    image::open(path)
}

pub fn load_texture<P>(
    file_name: P,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    ty: TextureType,
) -> Result<Texture, LoadTextureError>
where
    P: AsRef<std::path::Path>,
{
    let file_name = file_name.as_ref();
    let label = file_name.to_string_lossy();
    let data = load_binary(file_name)?;
    let texture = Texture::from_bytes_with(&data, device, queue, ty, &label)?;
    Ok(texture)
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum LoadTextureError {
    #[error(transparent)]
    TextureError(#[from] TextureError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
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
    MaterialWithoutTexture(String),
}

pub async fn load_model<P>(
    file_name: P,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
) -> Result<Model, ModelLoadError>
where
    P: AsRef<std::path::Path>,
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
            let abs_p = file_name
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(p);
            let mat_text = load_string(&abs_p)
                .inspect_err(|err| error!(%err, path = ?abs_p, "Failed to load material"))
                .map_err(|_| LoadError::OpenFileFailed)?;
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .map_err(ModelLoadError::LoadModelError)?;

    let mut materials = Vec::new();
    for m in obj_materials.map_err(ModelLoadError::LoadMaterialError)? {
        // Load the diffuse map
        let diffuse_texture = m
            .diffuse_texture
            .ok_or_else(|| ModelLoadError::MaterialWithoutTexture(m.name.clone()))?;
        let diffuse_texture = file_name
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(diffuse_texture);
        let diffuse_texture = load_texture(&diffuse_texture, device, queue, TextureType::Diffuse)
            .inspect_err(
            |err| error!(%err, path = ?diffuse_texture, "Failed to load diffuse texture"),
        )?;

        // Load the normal map
        let normal_texture = m
            .normal_texture
            .ok_or_else(|| ModelLoadError::MaterialWithoutTexture(m.name.clone()))?;
        let normal_texture = file_name
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(normal_texture);
        let normal_texture = load_texture(&normal_texture, device, queue, TextureType::Normal)
            .inspect_err(
                |err| error!(%err, path = ?normal_texture, "Failed to load diffuse texture"),
            )?;

        materials.push(Material::new(
            &device,
            &m.name,
            diffuse_texture,
            normal_texture,
            layout,
            sampler,
        ));
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    if m.mesh.normals.is_empty() {
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [0.0, 0.0, 0.0],
                            tangent: [0.0; 3],
                            bitangent: [0.0; 3],
                        }
                    } else {
                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [
                                m.mesh.texcoords[i * 2],
                                1.0 - m.mesh.texcoords[i * 2 + 1],
                            ],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                            tangent: [0.0; 3],
                            bitangent: [0.0; 3],
                        }
                    }
                })
                .collect::<Vec<_>>();

            let indices = &m.mesh.indices;
            let mut triangles_included = vec![0; vertices.len()];

            // Calculate tangents and bitangents. We're going to
            // use the triangles, so we need to loop through the
            // indices in chunks of 3
            for c in indices.chunks(3) {
                let v0 = vertices[c[0] as usize];
                let v1 = vertices[c[1] as usize];
                let v2 = vertices[c[2] as usize];

                let pos0: cgmath::Vector3<_> = v0.position.into();
                let pos1: cgmath::Vector3<_> = v1.position.into();
                let pos2: cgmath::Vector3<_> = v2.position.into();

                let uv0: cgmath::Vector2<_> = v0.tex_coords.into();
                let uv1: cgmath::Vector2<_> = v1.tex_coords.into();
                let uv2: cgmath::Vector2<_> = v2.tex_coords.into();

                // Calculate the edges of the triangle
                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;

                // This will give us a direction to calculate the tangent and bitangent
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;

                // Solving the following system of equations will give us the tangent and bitangent.
                //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
                //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
                // Luckily, the place I found this equation provided the solution!
                let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                // We flip the bitangent to enable right-handed normal
                // maps with wgpu texture coordinate system
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                // We'll use the same tangent/bitangent for each vertex in the triangle
                vertices[c[0] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[0] as usize].tangent)).into();
                vertices[c[1] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[1] as usize].tangent)).into();
                vertices[c[2] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[2] as usize].tangent)).into();

                vertices[c[0] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[0] as usize].bitangent)).into();
                vertices[c[1] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[1] as usize].bitangent)).into();
                vertices[c[2] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[2] as usize].bitangent)).into();

                // Used to average the tangents/bitangents
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average the tangents/bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f32;
                let v = &mut vertices[i];
                v.tangent = (cgmath::Vector3::from(v.tangent) * denom).into();
                v.bitangent = (cgmath::Vector3::from(v.bitangent) * denom).into();
            }

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
