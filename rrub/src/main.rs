#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod error;
mod firmware;
mod parser;
mod scheduler;

extern crate alloc;

use core::time::Duration;

use conquer_once::spin::OnceCell;
use simple_alloc::bump_alloc::LocklessBumpAlloc;

use crate::{error::RrubError, firmware::Firmware};

const NUM_HEAP_PAGES: usize = 32768;
static HEAP_START: OnceCell<usize> = OnceCell::uninit();

#[global_allocator]
static ALLOCATOR: LocklessBumpAlloc = LocklessBumpAlloc::new();

#[cfg(feature = "uefi")]
mod uefi_entry {
    use uefi::{Status, boot::stall, entry, println};

    use super::*;
    use crate::firmware::UefiFirmware;

    #[entry]
    fn uefi_entry() -> Status {
        match main::<UefiFirmware>() {
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
}

fn main<T: Firmware>() -> Result<(), RrubError> {
    let fw = T::init()?;

    //let mut fb = fw.init_fb(720, 480)?;

    return Ok(());
}
