use core::ptr::NonNull;

use uefi::boot::{AllocateType, MemoryType, allocate_pages, free_pages};

use crate::mem::FirmwareMemory;

pub struct UefiMemory;

impl FirmwareMemory for UefiMemory {
    fn allocate_pages(count: usize) -> NonNull<u8> {
        return allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, count)
            .expect("Unable to allocate pages for heap");
    }

    unsafe fn deallocate_pages(ptr: NonNull<u8>, count: usize) {
        unsafe {
            free_pages(ptr, count).expect("Pages to deallocate are not found");
        }
    }
}
