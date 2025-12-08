use alloc::{vec, vec::Vec};
use core::ptr::copy_nonoverlapping;

// use embedded_graphics::{
//     Pixel,
//     pixelcolor::Rgb888,
//     prelude::{DrawTarget, OriginDimensions, Point, RgbColor, Size},
// };
use uefi::{
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    proto::console::gop::{GraphicsOutput, PixelFormat as UefiPixelFormat},
};

use crate::{
    error::RrubError,
    firmware::framebuffer::{FrameBuffer, PixelFormat},
};

impl From<UefiPixelFormat> for PixelFormat {
    fn from(format: UefiPixelFormat) -> Self {
        match format {
            UefiPixelFormat::Rgb => Self::Rgb,
            UefiPixelFormat::Bgr => Self::Bgr,
            _ => Self::NotSupported,
        }
    }
}

pub struct UefiDisplay {
    fb_ptr: *mut u8,
    fb_size: usize,
    fb_format: UefiPixelFormat,
    buffer: Vec<u8>,
    stride: usize,
    size: (usize, usize), // width, height
}

impl FrameBuffer for UefiDisplay {
    fn init_fb_backend(width: usize, height: usize) -> Result<Self, RrubError> {
        let gop_handle = get_handle_for_protocol::<GraphicsOutput>()?;
        let mut gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

        let cur_res = gop.current_mode_info().resolution();
        if cur_res != (width, height) {
            let target_mode = gop
                .modes()
                .find(|mode| mode.info().resolution() == (width, height));
            if let Some(mode) = target_mode {
                gop.set_mode(&mode)?;
            }
        }

        let new_mode = gop.current_mode_info();
        let mut fb = gop.frame_buffer();
        let fb_size = fb.size();

        return Ok(UefiDisplay {
            fb_ptr: fb.as_mut_ptr(),
            fb_size,
            fb_format: new_mode.pixel_format(),
            buffer: vec![0u8; fb_size],
            stride: new_mode.stride(),
            size: (width, height),
        });
    }

    fn width(&self) -> usize {
        return self.size.0;
    }

    fn height(&self) -> usize {
        return self.size.1;
    }

    fn stride(&self) -> usize {
        return self.stride;
    }

    fn pixel_format(&self) -> PixelFormat {
        match self.fb_format {
            UefiPixelFormat::Rgb => PixelFormat::Rgb,
            UefiPixelFormat::Bgr => PixelFormat::Bgr,
            _ => PixelFormat::NotSupported,
        }
    }

    fn buffer(&mut self) -> &mut Vec<u8> {
        return &mut self.buffer;
    }

    fn flush(&mut self) {
        assert!(
            self.fb_size == self.buffer.len(),
            "UEFI Frame buffer size not equal to flushed buffer size"
        );
        unsafe {
            copy_nonoverlapping(self.buffer.as_ptr(), self.fb_ptr, self.fb_size);
        }
    }
}
