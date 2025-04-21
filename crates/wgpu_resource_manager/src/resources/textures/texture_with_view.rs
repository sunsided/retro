use std::borrow::Borrow;
use std::ops::Deref;
use wgpu::{Texture, TextureView};
use crate::ResourceData;
use crate::resources::textures::TextureResourceData;

/// A texture with a view.
#[derive(Debug, Clone)]
pub struct TextureWithView {
    texture: Texture,
    view: TextureView,
}

impl ResourceData for TextureWithView {}

impl TextureResourceData for TextureWithView {
    #[inline]
    fn texture(&self) -> &Texture {
        Self::texture(self)
    }

    #[inline]
    fn view(&self) -> &TextureView {
        Self::view(self)
    }
}

impl TextureWithView {
    #[inline]
    pub const fn new(texture: Texture, view: TextureView) -> Self {
        Self { texture, view }
    }

    #[inline]
    pub fn from_texture(texture: Texture) -> Self {
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view }
    }

    #[inline]
    pub const fn texture(&self) -> &Texture {
        &self.texture
    }

    #[inline]
    pub const fn view(&self) -> &TextureView {
        &self.view
    }
}

impl AsRef<Texture> for TextureWithView {
    #[inline]
    fn as_ref(&self) -> &Texture {
        Self::texture(self)
    }
}

impl AsRef<TextureView> for TextureWithView {
    #[inline]
    fn as_ref(&self) -> &TextureView {
        Self::view(self)
    }
}

impl Borrow<Texture> for TextureWithView {
    #[inline]
    fn borrow(&self) -> &Texture {
        Self::texture(self)
    }
}

impl Borrow<TextureView> for TextureWithView {
    #[inline]
    fn borrow(&self) -> &TextureView {
        Self::view(self)
    }
}

impl Deref for TextureWithView {
    type Target = Texture;

    #[inline]
    fn deref(&self) -> &Self::Target {
        Self::texture(self)
    }
}

impl From<Texture> for TextureWithView {
    #[inline]
    fn from(value: Texture) -> Self {
        Self::from_texture(value)
    }
}
