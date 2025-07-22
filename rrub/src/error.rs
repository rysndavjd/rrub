#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RrubError {
    CommandError(&'static str),
    UefiError(uefi::Error),
    UnsupportedResolution(usize, usize),
    UnsupportedColourFormat(&'static str),
    CalculationError(&'static str),
    OutOfBounds(&'static str),
    DrawingError(&'static str),
}

impl From<uefi::Error> for RrubError {
    fn from(err: uefi::Error) -> Self {
        RrubError::UefiError(err)
    }
}
