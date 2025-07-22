#![no_std]
#![no_main]

mod error;
mod shell;
mod display;
mod usb;

use core::panic::PanicInfo;
#[cfg(feature = "uefi")]
use uefi::prelude::*;
use crate::shell::start_shell;
use uefi::system::with_stdin;
use uefi::println;
use crate::usb::usb_test;

extern crate alloc;

#[cfg(feature = "uefi")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{:?}", _info);
    loop {}
}

#[cfg(feature = "uefi")]
#[entry]
pub fn entry() -> Status {
    use uefi::boot::stall;

    usb_test().unwrap();
    stall(100_000_000);

    Status::SUCCESS
}

#[cfg(feature = "bios")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "bios")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    bios::entry()
}
