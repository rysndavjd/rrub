use alloc::vec::Vec;
use core::{
    alloc::Layout,
    ptr::{NonNull, copy},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use bitflags::bitflags;
use log::error;
use simple_alloc::BAllocator;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unalign};

use crate::{ALLOCATOR, HEAP_START, NUM_HEAP_PAGES, error::RrubError};

pub const PAGE_SIZE: usize = 4096;

pub enum AllocationType {
    AnyPages,
    Address(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Available,
    Reserved,
    Acpi,
    Nvs,
    Unusable,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
    pub region_type: MemoryType,
}

pub type MemoryMap = Vec<MemoryRegion>;

bitflags! {
    #[derive(Debug)]
    pub struct MemAttr: u8 {
        const Execute = 1 << 0;
        const Write = 1 << 1;
        const Read = 1 << 2;
    }
}

#[derive(Debug)]
pub struct MemoryArea<T> {
    start: NonNull<T>, // base
    page_count: usize, // number of 4096 byte pages "T" takes, padding if needed
}

impl<T> Drop for MemoryArea<T> {
    fn drop(&mut self) {
        unsafe {
            match ALLOCATOR.try_deallocate(
                NonNull::new_unchecked(self.as_ptr() as *mut u8),
                Layout::for_value(self),
            ) {
                Ok(_) => return,
                Err(e) => error!(
                    "Error deallocating dropped memory: {:#X}, {:?}",
                    self.start.addr(),
                    e
                ),
            }
        }
    }
}

impl<T> MemoryArea<T> {
    // pub fn new(addr: usize, mem_attrs: MemAttr) -> Result<MemoryRegion<T>, RrubError> {
    //     if (addr & 0xFFF) != 0 {
    //         return Err(RrubError::UnalignedMemoryAddress);
    //     }
    //     let page_count = size_of::<T>().div_ceil(PAGE_SIZE);

    //     //let start = B::allocate(addr, page_count)?;

    //     return Ok(MemoryRegion {
    //         start,
    //         page_count,
    //     });
    // }

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
                (self.start.addr().get() + offset) as *mut u8,
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

    pub fn zero(&mut self) {
        unsafe {
            self.as_ptr().write_bytes(0, 1);
        }
    }
}
