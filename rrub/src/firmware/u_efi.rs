use core::ptr::NonNull;

use uefi::{
    Status,
    runtime::{ResetType, reset},
};

use crate::{
    RrubError,
    firmware::{
        Firmware,
        framebuffer::GraphicalDisplay,
        input::InputHandle,
        memory::MemoryMap,
        u_efi::{gop::UefiDisplay, input::UefiInput},
    },
};

mod gop;
mod input;
mod mem;

struct UefiFirmware {}

impl Firmware for UefiFirmware {
    type FB = UefiDisplay;
    type Input = UefiInput;

    fn init() -> Self {}

    fn init_input() -> Result<InputHandle<Self::Input>, RrubError> {}

    fn init_tty() {}

    fn init_fb() -> Result<GraphicalDisplay<Self::FB>, RrubError> {}

    fn get_memory_map(&self) -> MemoryMap {}

    fn allocate_pages(
        &mut self,
        allocation_type: super::memory::AllocationType,
        count: usize,
    ) -> Result<NonNull<u8>, crate::error::RrubError> {
    }

    unsafe fn deallocate_pages(&mut self, ptr: NonNull<u8>) -> Result<(), RrubError> {
        return Ok(());
    }

    fn handover() -> ! {
        loop {}
    }

    fn reboot() -> ! {
        reset(ResetType::COLD, Status::SUCCESS, None)
    }

    fn shutdown() -> ! {
        reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
    }
}
