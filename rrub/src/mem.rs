pub mod e820;
pub mod elf;
mod mem_bios;
#[cfg(feature = "uefi")]
mod mem_uefi;

use core::{
    marker::PhantomData,
    ptr::{NonNull, copy},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use bitflags::bitflags;
use log::error;
use simple_alloc::AllocInit;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unalign};

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

bitflags! {
    #[derive(Debug)]
    pub struct MemAttr: u8 {
        const Execute = 1 << 0;
        const Write = 1 << 1;
        const Read = 1 << 2;
    }
}

#[cfg(feature = "uefi")]
pub type Backend = crate::mem::mem_uefi::UefiMemory;

pub trait MemoryBackend<T> {
    fn allocate(addr: usize, page_count: usize) -> Result<NonNull<T>, RrubError>;
    unsafe fn deallocate(ptr: NonNull<T>, page_count: usize) -> Result<(), RrubError>;
    fn get_mem_attrs(addr: usize, page_count: usize) -> Result<MemAttr, RrubError>;
    unsafe fn update_mem_attrs(
        addr: usize,
        page_count: usize,
        new_attrs: &MemAttr,
        clear_attrs: &MemAttr,
    ) -> Result<(), RrubError>;
}

#[derive(Debug)]
pub struct MemoryRegion<T, B: MemoryBackend<T>> {
    start: NonNull<T>, // base
    page_count: usize, // number of 4096 byte pages "T" takes, padding if needed
    mem_attrs: MemAttr,
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
    pub fn new(addr: usize, mem_attrs: MemAttr) -> Result<MemoryRegion<T, B>, RrubError> {
        if (addr & 0xFFF) != 0 {
            return Err(RrubError::UnalignedMemoryAddress);
        }
        let page_count = size_of::<T>().div_ceil(PAGE_SIZE);

        let start = B::allocate(addr, page_count)?;

        //let old = B::get_mem_attrs(addr, page_count).unwrap();
        //unsafe {
        //    B::update_mem_attrs(addr, page_count, &mem_attrs, &old)?;
        //}

        return Ok(MemoryRegion {
            start,
            page_count,
            mem_attrs, 
            _backend: PhantomData,
        });
    }

    pub fn length_of_pages(&self) -> usize {
        return self.page_count * PAGE_SIZE;
    }

    pub fn length_of_data(&self) -> usize {
        return size_of::<T>();
    }

    pub fn as_ptr(&self) -> *mut T {
        return self.start.as_ptr();
    }

    pub fn as_ref(&self) -> &T {
        unsafe {
            return self.start.as_ref();
        }
    }

    pub fn as_mut(&mut self) -> &mut T {
        unsafe {
            return self.start.as_mut();
        }
    }

    pub fn read_ref<O: FromBytes + KnownLayout + Immutable>(
        &self,
        offset: usize,
    ) -> Result<&Unalign<O>, RrubError> {
        assert!((offset + size_of::<O>()) <= self.length_of_data());

        return Unalign::<O>::ref_from_bytes(unsafe {
            from_raw_parts(
                (self.start.as_ptr() as usize + offset) as *const u8,
                size_of::<O>(),
            )
        })
        .map_err(|_| RrubError::UnalignedMemoryAddress);
    }

    pub fn read_mut<O: FromBytes + IntoBytes + KnownLayout + Immutable>(
        &mut self,
        offset: usize,
    ) -> Result<&mut Unalign<O>, RrubError> {
        assert!((offset + size_of::<O>()) <= self.length_of_data());

        return Unalign::<O>::mut_from_bytes(unsafe {
            from_raw_parts_mut(
                (self.start.as_ptr() as usize + offset) as *mut u8,
                size_of::<O>(),
            )
        })
        .map_err(|_| RrubError::UnalignedMemoryAddress);
    }

    pub fn write<O: IntoBytes + KnownLayout + Immutable + ?Sized>(
        &mut self,
        value: &O,
        offset: usize,
    ) -> Result<(), RrubError> {
        let bytes = value.as_bytes();
        assert!((offset + bytes.len()) <= self.length_of_data());

        unsafe {
            copy(
                bytes.as_ptr(),
                (self.start.as_ptr() as *mut u8).add(offset),
                bytes.len(),
            );
        };
        return Ok(());
    }

    pub fn get_mem_attrs(addr: usize, page_count: usize) -> Result<MemAttr, RrubError> {
        return B::get_mem_attrs(addr, page_count);
    }

    pub unsafe fn update_mem_attrs(addr: usize, page_count: usize, new_attrs: &MemAttr, clear_attrs: &MemAttr) -> Result<(), RrubError> {
        unsafe {
            return B::update_mem_attrs(addr, page_count, new_attrs, clear_attrs);
        }
    }
}
