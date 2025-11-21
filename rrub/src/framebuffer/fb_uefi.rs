use alloc::{vec, vec::Vec};
use core::ptr::copy_nonoverlapping;

use embedded_graphics::prelude::Size;
use uefi::{
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    proto::console::gop::{GraphicsOutput, PixelFormat as UefiPixelFormat},
};

use crate::framebuffer::{FrameBufferBackend, PixelFormat};

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

impl UefiDisplay {
    pub fn init(resolution: (usize, usize)) -> UefiDisplay {
        let gop_handle =
            get_handle_for_protocol::<GraphicsOutput>().expect("Unable to get GOP handle");
        let mut gop = open_protocol_exclusive::<GraphicsOutput>(gop_handle)
            .expect("Unable to open GOP protocol");

        let cur_res = gop.current_mode_info().resolution();
        if cur_res != (resolution.0, resolution.1) {
            let target_mode = gop
                .modes()
                .find(|mode| mode.info().resolution() == (resolution.0, resolution.1));
            if let Some(mode) = target_mode {
                gop.set_mode(&mode)
                    .expect("Unable to set new GOP resolution");
            }
        }

        let new_mode = gop.current_mode_info();
        let mut fb = gop.frame_buffer();
        let fb_size = fb.size();

        return UefiDisplay {
            fb_ptr: fb.as_mut_ptr(),
            fb_size,
            fb_format: new_mode.pixel_format(),
            buffer: vec![0u8; fb_size],
            stride: new_mode.stride(),
            size: (resolution.0, resolution.1),
        };
    }
}

impl FrameBufferBackend for UefiDisplay {
    fn width(&self) -> usize {
        return self.size.0;
    }

    fn height(&self) -> usize {
        return self.size.1;
    }

    fn size(&self) -> Size {
        return Size::new(self.size.0 as u32, self.size.1 as u32);
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

    fn flush(&self) {
        assert!(
            self.fb_size == self.buffer.len(),
            "UEFI Frame buffer size not equal to flushed buffer size"
        );
        unsafe {
            copy_nonoverlapping(self.buffer.as_ptr(), self.fb_ptr, self.buffer.len());
        }
    }
}
