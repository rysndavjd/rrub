use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum RrubError {
    #[error("Command Error: {0}")]
    CommandError(&'static str),
    #[error("Uefi Error: {0}")]
    UefiError(#[from] uefi::Error),
    #[error("Unsupported resolution: {0}")]
    UnsupportedResolution(usize, usize),
    #[error("Unsupported Colour Format: {0}")]
    UnsupportedColourFormat(&'static str),
    #[error("Calculation Error: {0}")]
    CalculationError(&'static str),
    #[error("Out of Bounds: {0}")]
    OutOfBounds(&'static str),
    #[error("Drawing error: {0}")]
    DrawingError(&'static str),
}

