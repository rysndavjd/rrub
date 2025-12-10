use log::{LevelFilter, SetLoggerError, set_logger, set_max_level};

#[cfg(feature = "uefi")]
use crate::firmware::u_efi::LOGGER;

pub fn init_logger() -> Result<(), SetLoggerError> {
    set_logger(&LOGGER).map(|()| set_max_level(LevelFilter::Trace))
}
