#![no_main]
#![no_std]

use uefi::{prelude::*, println, boot};

mod config;
mod display;
mod error;
mod loaders;
mod shell;

use display::draw;
use crate::{display::set_resolution, shell::start_shell};
use uefi::proto::console::gop::GraphicsOutput;
use crate::display::UefiDisplay;

extern crate alloc;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    println!("Before!");

    //set_resolution(1920, 1080).unwrap();

    match uefi::system::with_stdin(start_shell) {
        Ok(_) => println!("Works"),
        Err(e) => println!("Error: {}", e)
    }

    boot::stall(120_000_000);
    Status::SUCCESS
}
