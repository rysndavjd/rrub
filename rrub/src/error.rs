#[cfg(feature = "uefi")]
use uefi::Error as FirmwareError;

// #[derive(Debug, Eq, PartialEq, Clone)]
// pub enum FirmwareError {
//     SecurityViolation,
//     DevieError,
//     Out
// }

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RrubError {
    CommandError,
    UnsupportedResolution(usize, usize),
    UnsupportedColourFormat,
    Overflow,
    DrawingError,
    MemoryFault,
    FirmwareError(FirmwareError),
    UnalignedMemoryAddress,
}

impl From<FirmwareError> for RrubError {
    fn from(error: FirmwareError) -> Self {
        RrubError::FirmwareError(error)
    }
}
