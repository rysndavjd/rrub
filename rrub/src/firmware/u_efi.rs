use core::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use log::LevelFilter;
use uefi::{
    Status,
    boot::{
        AllocateType as UefiAllocateType, MemoryType, allocate_pages, exit_boot_services,
        free_pages,
    },
    mem::memory_map::MemoryMap as UefiMemoryMap,
    runtime::{ResetType, reset},
};

use crate::{
    RrubError,
    firmware::{
        Firmware,
        framebuffer::{FrameBuffer, GraphicalDisplay},
        input::InputHandle,
        logger::init_logger,
        memory::{AllocationType, MemoryMap, MemoryRegion},
        u_efi::{gop::UefiDisplay, input::UefiInput, logger::UefiLogger},
    },
};

mod gop;
mod input;
mod logger;
mod mem;
mod panic;

#[cfg(debug_assertions)]
pub static LOGGER: UefiLogger = UefiLogger::new(LevelFilter::Trace);
#[cfg(not(debug_assertions))]
pub static LOGGER: UefiLogger = UefiLogger::new(LevelFilter::Error);

pub static BOOT_SERVICES_EXITED: AtomicBool = AtomicBool::new(false);

struct UefiFirmware {}

impl Firmware for UefiFirmware {
    type FB = UefiDisplay;
    type Input = UefiInput;

    fn init() -> Self {
        todo!()
    }

    fn init_input() -> Result<InputHandle<Self::Input>, RrubError> {
        todo!()
    }

    fn init_tty(columns: usize, rows: usize) {
        todo!()
    }

    fn init_fb(width: usize, height: usize) -> Result<GraphicalDisplay<Self::FB>, RrubError> {
        let backend = Self::FB::init_fb_backend(width, height)?;

        return Ok(GraphicalDisplay::new(backend));
    }

    fn get_memory_map(&self) -> MemoryMap {
        let uefi_map = unsafe { exit_boot_services(None) };

        BOOT_SERVICES_EXITED.store(true, Ordering::SeqCst);

        return uefi_map.entries().map(MemoryRegion::from).collect();
    }

    fn allocate_pages(
        &mut self,
        allocation_type: AllocationType,
        count: usize,
    ) -> Result<NonNull<u8>, RrubError> {
        let uefi_alloc = match allocation_type {
            AllocationType::AnyPages => UefiAllocateType::AnyPages,
            AllocationType::Address(addr) => UefiAllocateType::Address(addr),
        };

        let ptr = allocate_pages(uefi_alloc, MemoryType::LOADER_DATA, count)?;
        return Ok(ptr);
    }

    unsafe fn deallocate_pages(&mut self, ptr: NonNull<u8>, count: usize) -> Result<(), RrubError> {
        unsafe {
            free_pages(ptr, count)?;
        }
        return Ok(());
    }

    fn handover() -> ! {
        todo!()
    }

    fn reboot() -> ! {
        reset(ResetType::COLD, Status::SUCCESS, None)
    }

    fn shutdown() -> ! {
        reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
    }
}
