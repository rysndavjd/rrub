#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod error;
mod firmware;
mod loaders;
mod panic_handler;
mod scheduler;

extern crate alloc;

use core::time::Duration;

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;
use uefi::{
    Status,
    boot::stall,
    boot::{
        OpenProtocolAttributes, OpenProtocolParams, get_handle_for_protocol,
        open_protocol_exclusive,
    },
    entry, println,
    proto::loaded_image::LoadedImage,
};
use crate::{
    error::RrubError, firmware::Firmware, panic_handler::{Logger, init_logger}
};

const NUM_HEAP_PAGES: usize = 32768;
static HEAP_START: OnceCell<usize> = OnceCell::uninit();

#[global_allocator]
static ALLOCATOR: LocklessBumpAlloc = LocklessBumpAlloc::new();

#[cfg(debug_assertions)]
static LOGGER: Logger = Logger;

#[entry]
#[cfg(feature = "uefi")]
fn uefi_entry() -> Status {
    init_logger().unwrap();

    match main() {
        Ok(_) => {
            #[cfg(debug_assertions)]
            stall(Duration::from_mins(2));
            return Status::SUCCESS;
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            {
                println!("{:?}", e);
                stall(Duration::from_mins(2));
            }
            return Status::ABORTED;
        }
    }
}

fn main() -> Result<(), RrubError> {
    
    return Ok(());
}
