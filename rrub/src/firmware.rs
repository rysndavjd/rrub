mod framebuffer;
mod input;
mod memory;
mod u_efi;

use core::ptr::NonNull;

use crate::{
    error::RrubError,
    firmware::{
        framebuffer::{FrameBuffer, GraphicalDisplay},
        input::{InputBackend, InputHandle},
        memory::{AllocationType, MemoryInfo, MemoryMap},
    },
};

pub trait Firmware {
    type Input: InputBackend;
    type FB: FrameBuffer;

    fn init() -> Self;

    fn init_input() -> Result<InputHandle<Self::Input>, RrubError>;

    fn init_tty();
    fn init_fb() -> Result<GraphicalDisplay<Self::FB>, RrubError>;

    fn get_memory_map(&self) -> MemoryMap;
    fn allocate_pages(
        &mut self,
        allocation_type: AllocationType,
        count: usize,
    ) -> Result<NonNull<u8>, RrubError>;
    unsafe fn deallocate_pages(&mut self, ptr: NonNull<u8>) -> Result<(), RrubError>;

    fn handover() -> !;
    fn reboot() -> !;
    fn shutdown() -> !;
}
