use core::ptr::copy_nonoverlapping;
use alloc::vec::Vec;

use uefi::boot;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat, FrameBuffer, ModeInfo};
use embedded_graphics::{prelude::*, draw_target::DrawTarget, pixelcolor::Rgb888,
    primitives::Rectangle};

use crate::error::RrubError;

pub struct UefiDisplay {
    fb_ptr: *mut u8,
    fb_size: usize,
    fb_format: PixelFormat,
    buffer: Vec<u8>,
    stride: u32,
    size: (u32, u32),
}

impl UefiDisplay {
    pub fn new(
            mut frame_buffer: FrameBuffer,
            mode_info: ModeInfo,
        ) -> Result<Self, RrubError>
    {
        let (width, height) = (
            mode_info.resolution().0 as u32,
            mode_info.resolution().1 as u32,
        );

        let stride = mode_info.stride() as u32;

        let buf_len = width
            .checked_mul(height) // Area of display
            .and_then(|p| p.checked_mul(4)) // each pixel needs 4 bytes
            .ok_or(RrubError::CalculationError("Failed to calulate display buffer length."))?;

        let mut buffer = Vec::new();
        buffer.resize(buf_len as usize, 0);

        let pixel_format = mode_info.pixel_format();

        let mut display = UefiDisplay {
            fb_ptr: frame_buffer.as_mut_ptr(),
            fb_size: frame_buffer.size(),
            fb_format: pixel_format,
            buffer,
            stride,
            size: (width, height)
        };
        
        // Use white when starting framebuffer to know something is going on in debug
        #[cfg(debug_assertions)]
        display.fill_solid(
            &Rectangle::new(Point::zero(), Size::new(width, height)), 
            Rgb888::WHITE,
        )?;

        #[cfg(not(debug_assertions))]
        display.fill_solid(
            &Rectangle::new(Point::zero(), Size::new(width, height)), 
            Rgb888::BLACK,
        )?;

        Ok(display)
    }

    pub fn flush(&self) {
        assert!(self.fb_size == self.buffer.len(), "UEFI Frame buffer size not equal to flushed buffer size");
        unsafe {
            copy_nonoverlapping(self.buffer.as_ptr(), self.fb_ptr, self.buffer.len());
        }
    }
}

impl core::fmt::Debug for UefiDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UefiDisplay")
            .field("fb_ptr", &self.fb_ptr)
            .field("fb_size", &self.fb_size)
            .field("fb_format", &self.fb_format)
            .field("stride", &self.stride)
            .field("size", &self.size)
            .finish()
    }
}

impl OriginDimensions for UefiDisplay {
    fn size(&self) -> Size {
        Size::new(self.size.0 as u32, self.size.1 as u32)
    }
}

impl DrawTarget for UefiDisplay {
    type Color = Rgb888;
    type Error = RrubError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>> 
    {
        let (width, _) = self.size;
        let stride = self.stride as usize;
        let buf = &mut self.buffer;

        for Pixel(Point { x, y }, color) in pixels.into_iter() {
            if x < 0 || y < 0 {
                continue;
            }

            let (x, y): (usize, usize) = (x as usize, y as usize);

            if x >= width as usize {
                continue;
            }

            let idx = y
                .checked_mul(stride)
                .and_then(|row| row.checked_add(x))
                .and_then(|pix| pix.checked_mul(4))
                .ok_or(RrubError::OutOfBounds("index for pixel buffer"))?;

            let pixel_bytes = match self.fb_format {
                PixelFormat::Rgb => [color.r(), color.g(), color.b(), 0],
                PixelFormat::Bgr => [color.b(), color.g(), color.r(), 0],
                PixelFormat::Bitmask => return Err(RrubError::UnsupportedColourFormat("Bitmask")),
                PixelFormat::BltOnly => return Err(RrubError::UnsupportedColourFormat("Blt"))
            };

            buf[idx..idx + 4].copy_from_slice(&pixel_bytes);
        }
        self.flush();
        return Ok(());
    }
}

pub fn set_resolution(
        width: usize, 
        height: usize
    ) -> Result<(), RrubError>
{
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    let mode = gop.modes().find(|mode| {
        mode.info().resolution() == (width, height)
    }).ok_or(RrubError::UnsupportedResolution(width, height))?;

    gop.set_mode(&mode)?;

    return Ok(());
}
