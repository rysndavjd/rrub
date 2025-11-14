#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod error;
mod framebuffer;
mod mem;
mod panic_handler;
mod scheduler;
mod serial;
mod usb;

extern crate alloc;

use core::time::Duration;

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;
use uefi::{boot::stall, prelude::*};

use crate::{
    framebuffer::{GenericDisplay, UefiDisplay},
    mem::init_heap,
    panic_handler::{Logger, init_logger},
    scheduler::{Executor, Task},
};

const HEAP_PAGES: usize = 32768;
static HEAP_START: OnceCell<usize> = OnceCell::uninit();

#[global_allocator]
static ALLOCATOR: LocklessBumpAlloc = LocklessBumpAlloc::new();

#[cfg(debug_assertions)]
static LOGGER: Logger = Logger;

#[entry]
pub fn main() -> Status {
    #[cfg(debug_assertions)]
    init_logger().unwrap();
    init_heap().unwrap();

    //let mut executor = Executor::new();

    //let t = GenericDisplay::new(UefiDisplay::init((720, 480)));

    #[cfg(debug_assertions)]
    stall(Duration::from_mins(2));
    Status::SUCCESS
}
