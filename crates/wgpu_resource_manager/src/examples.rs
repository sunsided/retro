mod fallback_normal;

#[cfg(feature = "image")]
mod file_texture;

pub use self::fallback_normal::FallbackNormalTexture;

#[cfg(feature = "image")]
pub use self::file_texture::{FileTexture, FileTextureError};
