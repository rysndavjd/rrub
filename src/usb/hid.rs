use crate::usb::ToChar;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct KeyModifiers: u8 {
        const LEFT_CTRL = 0x01;
        const RIGHT_CTRL = 0x10;
        const LEFT_SHIFT = 0x02;
        const RIGHT_SHIFT = 0x20;
        const LEFT_ALT = 0x04;
        const RIGHT_ALT = 0x40;
        const LEFT_SUPER = 0x08;
        const RIGHT_SUPER = 0x80;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HidKeyboardReport {
    pub modifiers: KeyModifiers,
    pub reserved: u8,
    pub keycodes: [u8; 6],
}

impl HidKeyboardReport {
    pub fn new(data: [u8; 8]) -> HidKeyboardReport {
        HidKeyboardReport {
            modifiers: KeyModifiers::from_bits_retain(data[0]),
            reserved: data[1],
            keycodes: [data[2], data[3], data[4], data[5], data[6], data[7]],
        }
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers
            .intersects(KeyModifiers::LEFT_SHIFT | KeyModifiers::RIGHT_SHIFT)
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers
            .intersects(KeyModifiers::LEFT_CTRL | KeyModifiers::RIGHT_CTRL)
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers
            .intersects(KeyModifiers::LEFT_ALT | KeyModifiers::RIGHT_ALT)
    }

    pub fn is_super(&self) -> bool {
        self.modifiers
            .intersects(KeyModifiers::LEFT_SUPER | KeyModifiers::RIGHT_SUPER)
    }

    pub fn get<K: TryFrom<u8> + ToChar>(&self) -> [Option<char>; 6] {
        let shift = self.is_shift();

        let mut keys = [None; 6];

        #[allow(clippy::needless_range_loop)]
        for i in 0..=5usize {
            if self.keycodes[i] == 0 {
                continue;
            }
            if let Ok(key) = K::try_from(self.keycodes[i])
                && let Some(char) = key.char(shift)
            {
                keys[i] = Some(char);
            }
        }

        return keys;
    }
}
