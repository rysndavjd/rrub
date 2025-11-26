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

use core::{mem::offset_of, time::Duration};

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;
use uefi::println;
#[cfg(feature = "uefi")]
use uefi::{Status, boot::stall, entry};
use zerocopy::{}

use crate::{
    error::RrubError,
    loaders::linux::x86::{SetupHeader, Zeropage},
    mem::{Backend, MemoryRegion, init_heap},
    panic_handler::{Logger, init_logger},
};

const NUM_HEAP_PAGES: usize = 32768;
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

    let k_area = unsafe { MemoryRegion::<[u8; 77535936], Backend>::new(0xA000_0000, 1)? };

    k_area.write(0, KERNEL);

    let zeropage = unsafe { MemoryRegion::<Zeropage, Backend>::new(0xB000_0000, 1)? };

    zeropage.write(offset_of!(Zeropage, hdr), &KERNEL[0x1F1..0x1F1 + 4096]);

    println!("{:?}", zeropage.as_ref().hdr);

    stall(Duration::from_mins(2));
    return Ok(());
}
