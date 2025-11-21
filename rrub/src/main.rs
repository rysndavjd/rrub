#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod error;
mod framebuffer;
mod loaders;
mod mem;
mod panic_handler;
mod scheduler;
mod serial;
mod usb;

extern crate alloc;

use core::time::Duration;

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;
#[cfg(feature = "uefi")]
use uefi::{Status, boot::stall, entry};

use crate::{
    error::RrubError,
    mem::init_heap,
    panic_handler::{Logger, init_logger},
};

const HEAP_PAGES_COUNT: usize = 32768;
static HEAP_START: OnceCell<usize> = OnceCell::uninit();

#[global_allocator]
static ALLOCATOR: LocklessBumpAlloc = LocklessBumpAlloc::new();

#[cfg(debug_assertions)]
static LOGGER: Logger = Logger;

static KERNEL: &[u8; 77535936] = include_bytes!("../vmlinux");
//static INITRD: &[u8; 13144831] = include_bytes!("../initramfs.img");

#[cfg(feature = "uefi")]
#[entry]
fn uefi_entry() -> Status {
    init_logger().unwrap();
    init_heap();

    return match main() {
        Ok(_) => Status::SUCCESS,
        Err(_) => Status::ABORTED,
    };
}

fn main() -> Result<(), RrubError> {
    // unsafe {
    //     exit_boot_services(None);
    // };

    stall(Duration::from_mins(2));
    return Ok(());
}
