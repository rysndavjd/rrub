pub mod filesystem;
pub mod framebuffer;
pub mod input;
pub mod logger;
pub mod memory;
mod u_efi;

use core::ptr::NonNull;

#[cfg(feature = "uefi")]
pub use u_efi::UefiFirmware;

use crate::{
    error::RrubError,
    firmware::{
        filesystem::FilesystemsList,
        framebuffer::{FrameBuffer, GraphicalDisplay},
        input::{InputBackend, InputHandle},
        memory::{AllocationType, MemoryMap},
    },
};

pub trait Firmware: Sized {
    type Input: InputBackend;
    type FB: FrameBuffer;

    fn init() -> Result<Self, RrubError>;

    fn init_input(&self) -> Result<InputHandle<Self::Input>, RrubError>;

    fn init_tty(&self, columns: usize, rows: usize);
    fn init_fb(&self, width: usize, height: usize)
    -> Result<GraphicalDisplay<Self::FB>, RrubError>;

    fn get_memory_map(&self) -> MemoryMap;
    fn allocate_pages(
        &mut self,
        allocation_type: AllocationType,
        count: usize,
    ) -> Result<NonNull<u8>, RrubError>;
    unsafe fn deallocate_pages(&mut self, ptr: NonNull<u8>, count: usize) -> Result<(), RrubError>;

    fn get_filesystems(&self) -> Result<FilesystemsList, RrubError>;

    fn handover(self) -> !;
    fn reboot(self) -> !;
    fn shutdown(self) -> !;
}
