//mod fb_uefi;
mod vga;

use alloc::vec::Vec;

use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, Point, RgbColor, Size},
    primitives::Rectangle,
};
// #[cfg(feature = "uefi")]
// pub use fb_uefi::UefiDisplay;

use crate::error::RrubError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PixelFormat {
    Rgb,
    Bgr,
    NotSupported,
}

pub trait FrameBufferBackend {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> Size;
    fn stride(&self) -> usize;
    fn pixel_format(&self) -> PixelFormat;
    fn buffer(&mut self) -> &mut Vec<u8>;
    fn flush(&self);
}

pub struct GenericDisplay<B: FrameBufferBackend> {
    backend: B,
}

impl<B: FrameBufferBackend> GenericDisplay<B> {
    pub fn new(backend: B) -> Self {
        let size = backend.size();

        let mut framebuffer = GenericDisplay { backend };

        // Use white when starting framebuffer to know something is going on in debug
        #[cfg(debug_assertions)]
        framebuffer
            .fill_solid(&Rectangle::new(Point::zero(), size), Rgb888::WHITE)
            .expect("Unable to fill screen white.");

        #[cfg(not(debug_assertions))]
        framebuffer
            .fill_solid(&Rectangle::new(Point::zero(), size), Rgb888::BLACK)
            .expect("Unable to fill screen black.");

        framebuffer
    }
}

impl<B: FrameBufferBackend> OriginDimensions for GenericDisplay<B> {
    fn size(&self) -> Size {
        self.backend.size()
    }
}

impl<B: FrameBufferBackend> DrawTarget for GenericDisplay<B> {
    type Color = Rgb888;
    type Error = RrubError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width = self.backend.width();
        let stride = self.backend.stride();

        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            if x < 0 || y < 0 {
                continue;
            }

            let (x, y): (usize, usize) = (x as usize, y as usize);

            if x >= width {
                continue;
            }

            let idx = y
                .checked_mul(stride)
                .and_then(|row| row.checked_add(x))
                .and_then(|pix| pix.checked_mul(4))
                .ok_or(RrubError::Overflow)?;

            let pixel_bytes = match self.backend.pixel_format() {
                PixelFormat::Rgb => [color.r(), color.g(), color.b(), 0],
                PixelFormat::Bgr => [color.b(), color.g(), color.r(), 0],
                PixelFormat::NotSupported => {
                    return Err(RrubError::UnsupportedColourFormat);
                }
            };

            self.backend.buffer()[idx..idx + 4].copy_from_slice(&pixel_bytes);
        }
        self.backend.flush();
        return Ok(());
    }
}
