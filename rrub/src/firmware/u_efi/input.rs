use crate::firmware::input::InputBackend;

pub struct UefiInput {}

impl InputBackend for UefiInput {
    fn init_input_backend() -> Result<Self, crate::error::RrubError> {
        return Ok(UefiInput {});
    }

    fn read_key(&self) -> Option<crate::firmware::input::Key> {
        return None;
    }
}
