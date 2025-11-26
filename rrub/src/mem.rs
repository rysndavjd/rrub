pub mod e820;
pub mod elf;
pub mod mem_bios;
#[cfg(feature = "uefi")]
mod mem_uefi;

use core::{marker::PhantomData, ptr::NonNull, slice::from_raw_parts};

use log::error;
use simple_alloc::AllocInit;

use crate::{ALLOCATOR, HEAP_START, NUM_HEAP_PAGES, error::RrubError};

pub const PAGE_SIZE: usize = 4096;

// pub unsafe fn print_mem(heap: *const u8, len: usize) {
//     unsafe {
//         for i in 0..len {
//             if i % 16 == 0 {
//                 print!("\n{:08x}: ", i);
//             }
//             print!("{:02x} ", *heap.add(i));
//         }
//         println!();
//     }
// }

pub fn init_heap() {
    #[cfg(feature = "uefi")]
    {
        use uefi::boot::{AllocateType, MemoryType, allocate_pages};

        let ptr = allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            NUM_HEAP_PAGES,
        )
        .expect("Unable to allocate memory for heap.");

        HEAP_START.init_once(|| ptr.as_ptr() as usize);

        unsafe {
            ALLOCATOR.init(ptr.as_ptr() as usize, NUM_HEAP_PAGES * PAGE_SIZE);
        }
    }
}

#[cfg(feature = "uefi")]
pub type Backend = crate::mem::mem_uefi::UefiMemory;

pub trait MemoryBackend<T> {
    fn allocate(addr: u32, page_count: usize) -> Result<NonNull<T>, RrubError>;
    unsafe fn deallocate(ptr: NonNull<T>, page_count: usize) -> Result<(), RrubError>;
}

#[derive(Debug)]
pub struct MemoryRegion<T, B: MemoryBackend<T>> {
    start: NonNull<T>,        // base
    object_count: usize,      // number of "T" objects in this memory region
    page_count: usize,        // number of 4096 byte pages these "T" objects take, padding if needed
    _backend: PhantomData<B>, // backend to use, uefi or bios
}

impl<T, B: MemoryBackend<T>> Drop for MemoryRegion<T, B> {
    fn drop(&mut self) {
        unsafe {
            match B::deallocate(self.start, self.page_count) {
                Ok(_) => {}
                Err(e) => error!(
                    "Error deallocating dropped memory: {:#X}, {:?}",
                    self.start.addr(),
                    e
                ),
            };
        }
    }
}

impl<T, B: MemoryBackend<T>> MemoryRegion<T, B> {
    pub unsafe fn new(addr: u32, count: usize) -> Result<MemoryRegion<T, B>, RrubError> {
        if (addr & 0xFFF) != 0 {
            return Err(RrubError::UnalignedMemoryAddress);
        }
        let page_count = (size_of::<T>() * count).div_ceil(PAGE_SIZE);

        let start = B::allocate(addr, page_count)?;

        return Ok(MemoryRegion {
            start,
            object_count: count,
            page_count,
            _backend: PhantomData,
        });
    }

    pub fn length_of_pages(&self) -> usize {
        return self.page_count * PAGE_SIZE;
    }

    pub fn length_of_data(&self) -> usize {
        return self.object_count * size_of::<T>();
    }

    pub fn as_ref(&self) -> &T {
        unsafe { return self.start.as_ref() }
    }

    pub fn as_mut(&mut self) -> &mut T {
        unsafe { return self.start.as_mut() }
    }

    pub fn as_slice<O>(&self, offset: usize, length: usize) -> &[O] {
        assert!((offset + (length * size_of::<O>())) <= self.length_of_data());
        unsafe { from_raw_parts((self.start.as_ptr() as usize + offset) as *const O, length) }
    }

    pub fn read<O: Copy>(&self, offset: usize) -> O {
        assert!((offset + size_of::<O>()) < self.length_of_data());
        unsafe { *((self.start.as_ptr() as usize + offset) as *const O) }
    }

    pub fn write<O>(&self, offset: usize, value: O) {
        assert!((offset + size_of::<O>()) < self.length_of_data());
        unsafe {
            *((self.start.as_ptr() as usize + offset) as *mut O) = value;
        }
    }
}
