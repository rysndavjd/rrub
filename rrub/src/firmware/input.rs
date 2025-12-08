use crate::error::RrubError;

/*
 * https://en.wikipedia.org/wiki/C0_and_C1_control_codes
*/

#[non_exhaustive]
pub enum ControlChar {
    Backspace,    // \b
    Tab,          // \t
    LineFeed,     // \n
    VerticalTab,  // \v
    FormFeed,     // \f
    Return,       // \r
    EscapePrefix, // \e
}

#[non_exhaustive]
pub enum SpecialKey {
    Up,
    Down,
    Right,
    Left,
    Home,
    End,
    Insert,
    Delete,
    PageUp,
    PageDown,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Escape,
}

pub enum Key {
    Printable(char),
    ControlChar(ControlChar),
    SpecialKey(SpecialKey),
}

pub trait InputBackend: Sized {
    fn init_input_backend() -> Result<Self, RrubError>;
    fn read_key(&self) -> Option<Key>;
}

pub struct InputHandle<B: InputBackend> {
    backend: B,
}
