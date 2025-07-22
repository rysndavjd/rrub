use core::ptr::copy_nonoverlapping;
use alloc::vec::Vec;
use crate::error::RrubError;

use embedded_graphics::{prelude::*, draw_target::DrawTarget, pixelcolor::Rgb888,
    primitives::Rectangle};

#[derive(Debug)]
#[repr(C, packed)]
struct VbeInfoBlock {
    vbe_signature: [u8; 4],
    vbe_version: u16,
    oem_string_ptr: [u16; 2],
    capabilities: [u8; 4],
    video_mode_ptr: [u16; 2],
    total_memory: u16,
    reserved: [u8; 492],
}

#[derive(Debug)]
#[repr(C, packed)]
struct VbeModeInfoStructure {
    attributes: u16,
    window_a: u8,
    window_b: u8,
    granularity: u16,
    window_size: u16,
    segment_a: u16,
    segment_b: u16,
    win_func_ptr: u32,
    pitch: u16,
    width: u16,
    height: u16,
    w_char: u8,
    y_char: u8,
    planes: u8,
    bpp: u8,
    banks: u8,
    memory_model: u8,
    bank_size: u8,
    image_pages: u8,
    reserved0: u8,
    
    red_mask: u8,
    red_position: u8,
    green_mask: u8,
	green_position: u8,
	blue_mask: u8,
	blue_position: u8,
	reserved_mask: u8,
	reserved_position: u8,
	direct_color_attributes: u8,

    framebuffer: u32,
    off_screen_mem_off: u32,
    off_screen_mem_size: u16,
    reserved1: [u8; 206],
}

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    Rgb8888,
    Rgb888,
    Rgb565,
    Rgb555,
    Unknown,
}

impl PixelFormat {
    pub fn bits_per_pixel(self) -> usize {
        match self {
            PixelFormat::Rgb8888 => 32,
            PixelFormat::Rgb888 => 24,
            PixelFormat::Rgb565 => 16,
            PixelFormat::Rgb555 => 15,
            PixelFormat::Unknown => 0,
        }
    }

    pub fn bytes_per_pixel(self) -> usize {
        match self {
            PixelFormat::Rgb8888 => 4,
            PixelFormat::Rgb888 => 3,
            PixelFormat::Rgb565 => 2,
            PixelFormat::Rgb555 => 2,
            PixelFormat::Unknown => 0
        }
    }
}

pub struct VesaDisplay {
    fb_ptr: *mut u8,
    fb_size: usize,
    fb_format: PixelFormat,
    buffer: Vec<u8>,
    stride: u32,
    size: (u32, u32),
}

impl core::fmt::Debug for VesaDisplay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VesaDisplay")
            .field("fb_ptr", &self.fb_ptr)
            .field("fb_size", &self.fb_size)
            .field("fb_format", &self.fb_format)
            .field("stride", &self.stride)
            .field("size", &self.size)
            .finish()
    }
}

impl VesaDisplay {
    pub fn new(
            mut frame_buffer: VbeModeInfoStructure,
        ) -> Result<Self, RrubError>
    {
        todo!();
        //Ok()
    }

    pub fn flush(&self) {
        unsafe {
            copy_nonoverlapping(self.buffer.as_ptr(), self.fb_ptr, self.buffer.len());
        }
    }
}

impl OriginDimensions for VesaDisplay {
    fn size(&self) -> Size {
        Size::new(self.size.0 as u32, self.size.1 as u32)
    }
}

impl DrawTarget for VesaDisplay {
    type Color = Rgb888;
    type Error = RrubError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>> 
    {
        todo!();
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


            //let pixel_bytes = match self.fb_format {
            //    PixelFormat::Rgb888 => [color.r(), color.g(), color.b(), 0],
            //    PixelFormat::Rgb565 => ,
            //    PixelFormat::Rgb555 => ,
            //    PixelFormat::Unknown => return Err(RrubError::UnsupportedColourFormat("Blt"))
            //};
            //buf[idx..idx + 4].copy_from_slice(&pixel_bytes);
        }
        self.flush();
        return Ok(());
    }
}
