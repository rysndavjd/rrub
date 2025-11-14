use core::{panic::PanicInfo, time::Duration};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

#[cfg(feature = "uefi")]
pub use uefi::{boot::stall, print, println};

#[cfg(debug_assertions)]
use crate::LOGGER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info:?}");
    // if let Some(location) = info.location() {
    //     println!(
    //         "{info}" // "PANIC at line \"{}\", in file \"{}\"",
    //                  // location.line(),
    //                  // location.file(),
    //     );
    // } else {
    //     println!("PANIC?: {info}");
    // }

    stall(Duration::from_mins(1));
    loop {}
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Trace
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

#[cfg(debug_assertions)]
pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
