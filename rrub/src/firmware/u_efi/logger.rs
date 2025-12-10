use core::sync::atomic::Ordering;

use log::{LevelFilter, Log, Metadata, Record};
use uefi::println;

use crate::firmware::u_efi::BOOT_SERVICES_EXITED;

pub struct UefiLogger {
    set_level: LevelFilter,
}

impl UefiLogger {
    pub const fn new(level: LevelFilter) -> UefiLogger {
        return UefiLogger { set_level: level };
    }
}

impl Log for UefiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        return BOOT_SERVICES_EXITED.load(Ordering::SeqCst) && metadata.level() <= self.set_level;
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args())
        }
    }

    fn flush(&self) {}
}
