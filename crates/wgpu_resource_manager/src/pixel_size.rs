use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PixelSize {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
}

impl PixelSize {
    pub const fn new(width: NonZeroU32, height: NonZeroU32) -> Self {
        Self { width, height }
    }

    pub fn try_new<W, H>(width: W, height: H) -> Result<Self, PixelSizeError>
    where
        W: TryInto<NonZeroU32>,
        H: TryInto<NonZeroU32>,
    {
        let width = width
            .try_into()
            .map_err(|_| PixelSizeError::InvalidWidth(0))?;
        let height = height
            .try_into()
            .map_err(|_| PixelSizeError::InvalidHeight(0))?;
        Ok(Self::new(width, height))
    }

    pub const fn width(&self) -> u32 {
        self.width.get()
    }

    pub const fn height(&self) -> u32 {
        self.height.get()
    }
}

#[cfg(feature = "winit")]
impl TryFrom<winit::dpi::PhysicalSize<u32>> for PixelSize {
    type Error = PixelSizeError;

    fn try_from(value: winit::dpi::PhysicalSize<u32>) -> Result<Self, Self::Error> {
        let width = value
            .width
            .try_into()
            .map_err(|_| PixelSizeError::InvalidWidth(value.width))?;
        let height = value
            .height
            .try_into()
            .map_err(|_| PixelSizeError::InvalidHeight(value.height))?;
        Ok(Self::new(width, height))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PixelSizeError {
    #[error("The specified width is invalid: {0}")]
    InvalidWidth(u32),
    #[error("The specified height is invalid: {0}")]
    InvalidHeight(u32),
}
