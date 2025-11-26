use core::ptr::NonNull;

use uefi::boot::{AllocateType, MemoryType, allocate_pages, free_pages};

use crate::{error::RrubError, mem::MemoryBackend};

#[derive(Debug)]
pub struct UefiMemory {}

impl<T> MemoryBackend<T> for UefiMemory {
    fn allocate(addr: u32, page_count: usize) -> Result<NonNull<T>, RrubError> {
        assert!((addr & 0xFFF) == 0, "Addr not aligned to 4096 bytes");
        let ptr = allocate_pages(
            AllocateType::Address(addr as u64),
            MemoryType::custom(addr), // FIX later, could overflow
            page_count,
        )?;

        return Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut T) });
    }

    unsafe fn deallocate(ptr: NonNull<T>, page_count: usize) -> Result<(), RrubError> {
        unsafe {
            free_pages(NonNull::new_unchecked(ptr.as_ptr() as *mut u8), page_count)?;
        }
        return Ok(());
    }
}
