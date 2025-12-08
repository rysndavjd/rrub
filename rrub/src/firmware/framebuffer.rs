use alloc::vec::Vec;

use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, Point, RgbColor, Size},
    primitives::Rectangle,
};

use crate::error::RrubError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PixelFormat {
    Rgb,
    Bgr,
    NotSupported,
}

pub trait FrameBuffer: Sized {
    fn init_fb_backend(width: usize, height: usize) -> Result<Self, RrubError>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn stride(&self) -> usize;
    fn pixel_format(&self) -> PixelFormat;
    fn buffer(&mut self) -> &mut Vec<u8>;
    fn flush(&mut self);
}

pub struct GraphicalDisplay<B: FrameBuffer> {
    backend: B,
}

impl<B: FrameBuffer> GraphicalDisplay<B> {
    fn new(backend: B) -> Self {
        let size = Size {
            width: backend.width() as u32,
            height: backend.height() as u32,
        };

        let mut framebuffer = GraphicalDisplay { backend };

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

impl<B: FrameBuffer> OriginDimensions for GraphicalDisplay<B> {
    fn size(&self) -> Size {
        Size {
            width: self.backend.width() as u32,
            height: self.backend.height() as u32,
        }
    }
}

impl<B: FrameBuffer> DrawTarget for GraphicalDisplay<B> {
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
