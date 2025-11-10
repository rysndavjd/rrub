#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod error;
mod framebuffer;
mod mem;
mod panic_handler;
mod serial;
mod usb;

extern crate alloc;

use core::time::Duration;

use crate::{
    framebuffer::{GenericDisplay, UefiDisplay},
    mem::{init_heap, print_mem},
};
use alloc::string::String;
use conquer_once::spin::OnceCell;
use simple_alloc::{common::Locked, spin_lock::linked_list_alloc::LinkedListAlloc};
use uefi::{
    boot::{MemoryDescriptor, MemoryType, memory_map, stall},
    prelude::*,
    print as uefi_print, println as uefi_println,
};

const HEAP_PAGES: usize = 8192; // 4096 * 4096 bytes avaliable for allocation
static HEAP_START: OnceCell<usize> = OnceCell::uninit();

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAlloc> = Locked::new(LinkedListAlloc::new());

#[entry]
pub fn main() -> Status {
    init_heap().unwrap();

    // unsafe {
    //     print_mem(*HEAP_START.lock() as *const u8, HEAP_PAGES * 4096);
    // }

    let t = GenericDisplay::new(UefiDisplay::init((720, 480)));

    stall(Duration::from_secs(120));
    Status::SUCCESS
}
