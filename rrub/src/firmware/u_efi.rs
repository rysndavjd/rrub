use core::{
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use log::LevelFilter;
use simple_alloc::AllocInit;
use uefi::{
    Status,
    boot::{
        AllocateType as UefiAllocateType, AllocateType, MemoryType, allocate_pages,
        exit_boot_services, free_pages,
    },
    mem::memory_map::MemoryMap as UefiMemoryMap,
    runtime::{ResetType, reset},
};

use crate::{
    ALLOCATOR, HEAP_START, NUM_HEAP_PAGES, RrubError,
    firmware::{
        Firmware,
        filesystem::FilesystemsList,
        framebuffer::{FrameBuffer, GraphicalDisplay},
        input::InputHandle,
        logger::init_logger,
        memory::{AllocationType, MemoryMap, MemoryRegion, PAGE_SIZE},
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

pub struct UefiFirmware {}

impl Firmware for UefiFirmware {
    type FB = UefiDisplay;
    type Input = UefiInput;

    fn init() -> Result<Self, RrubError> {
        if init_logger().is_err() {
            return Err(Status::LOAD_ERROR.into());
        };

        let heap_ptr = allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            NUM_HEAP_PAGES,
        )?;

        HEAP_START.init_once(|| heap_ptr.addr().get());

        unsafe {
            ALLOCATOR.init(heap_ptr.addr().get(), NUM_HEAP_PAGES * PAGE_SIZE);
        }

        return Ok(UefiFirmware {});
    }

    fn init_input(&self) -> Result<InputHandle<Self::Input>, RrubError> {
        todo!()
    }

    fn init_tty(&self, columns: usize, rows: usize) {
        todo!()
    }

    fn init_fb(
        &self,
        width: usize,
        height: usize,
    ) -> Result<GraphicalDisplay<Self::FB>, RrubError> {
        let backend = Self::FB::init_fb_backend(width, height)?;

        return Ok(GraphicalDisplay::new(backend));
    }

    fn get_memory_map(&self) -> MemoryMap {
        BOOT_SERVICES_EXITED.store(true, Ordering::SeqCst);

        let uefi_map = unsafe { exit_boot_services(None) };

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

    fn get_filesystems(&self) -> Result<FilesystemsList, RrubError> {
        todo!()
    }

    fn handover(self) -> ! {
        todo!()
    }

    fn reboot(self) -> ! {
        reset(ResetType::COLD, Status::SUCCESS, None)
    }

    fn shutdown(self) -> ! {
        reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
    }
}
