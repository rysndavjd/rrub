#[cfg(feature = "uefi")]
use uefi::Error as FirmwareError;

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

#[cfg(feature = "uefi")]
mod uefi_errors {
    use uefi::{Error, Status};

    use super::*;

    impl From<Error> for RrubError {
        fn from(error: Error) -> Self {
            RrubError::FirmwareError(error)
        }
    }

    impl From<Status> for RrubError {
        fn from(status: Status) -> Self {
            RrubError::FirmwareError(Error::from(status))
        }
    }
}
