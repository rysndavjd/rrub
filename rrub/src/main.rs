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

use core::{
    ffi::c_void,
    mem::{offset_of, transmute},
    time::Duration,
};

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;
use uefi::table::system_table_raw;
#[cfg(feature = "uefi")]
use uefi::{
    Status,
    boot::stall,
    boot::{get_handle_for_protocol, open_protocol_exclusive},
    entry, println,
    proto::loaded_image::LoadedImage,
};
use uefi_raw::table::system::SystemTable;

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

static KERNEL: &[u8; 20532208] = include_bytes!("../vmlinuz");
//static INITRD: &[u8; 13144831] = include_bytes!("../initramfs.img");

#[cfg(feature = "uefi")]
#[entry]
fn uefi_entry() -> Status {
    init_logger().unwrap();
    init_heap();

    match main() {
        Ok(_) => {
            stall(Duration::from_mins(2));
            return Status::SUCCESS;
        }
        Err(_) => {
            stall(Duration::from_mins(2));
            return Status::ABORTED;
        }
    }
}

type HandoverFunc =
    extern "C" fn(uefi::boot::ScopedProtocol<LoadedImage>, *mut SystemTable, *mut u8);

fn main() -> Result<(), RrubError> {
    // unsafe {
    //     exit_boot_services(None);
    // };

    let mut k_area = MemoryRegion::<[u8; 20532208], Backend>::new(0xA000_0000)?;

    //k_area.write(0, KERNEL);

    k_area.write(KERNEL, 0)?;

    let mut zeropage = MemoryRegion::<Zeropage, Backend>::new(0xB000_0000)?;

    zeropage.write(
        &KERNEL[0x1f1..0x1f1 + size_of::<SetupHeader>()],
        offset_of!(Zeropage, hdr),
    )?;

    let handle = get_handle_for_protocol::<LoadedImage>()?;
    let mut image: uefi::boot::ScopedProtocol<LoadedImage> =
        open_protocol_exclusive::<LoadedImage>(handle)?;

    unsafe {
        let hf: HandoverFunc = transmute(
            k_area.as_ptr().addr() + (zeropage.as_ref().hdr.handover_offset.get() as usize) + 512,
        );
        hf(
            image,
            system_table_raw().unwrap().as_ptr(),
            zeropage.as_ptr() as *mut u8,
        );
    }

    return Ok(());
}
