mod mem_bios;
mod mem_uefi;

use crate::{
    ALLOCATOR, HEAP_PAGES, HEAP_START,
    error::RrubError,
    panic_handler::{print, println},
};
use core::ptr::NonNull;

// These functions are indented to panic if allocating or deallocating a page fails.
trait FirmwareMemory {
    fn allocate_pages(count: usize) -> NonNull<u8>;
    unsafe fn deallocate_pages(ptr: NonNull<u8>, count: usize);
}

pub unsafe fn print_mem(heap: *const u8, len: usize) {
    unsafe {
        for i in 0..len {
            if i % 16 == 0 {
                print!("\n{:08x}: ", i);
            }
            print!("{:02x} ", *heap.add(i));
        }
        println!();
    }
}

pub fn init_heap() -> Result<(), RrubError> {
    #[cfg(feature = "uefi")]
    {
        use crate::mem::mem_uefi::UefiMemory;

        let ptr = UefiMemory::allocate_pages(HEAP_PAGES);
        HEAP_START.init_once(|| ptr.as_ptr() as usize);

        unsafe {
            ALLOCATOR
                .lock()
                .init(ptr.as_ptr() as usize, HEAP_PAGES * 4096);
        }
        return Ok(());
    }
}
