use uefi::proto::console::serial::{
    ControlBits as UefiControlBits, IoMode, Parity as UefiParity, Serial, StopBits as UefiStopBits,
};

use crate::{
    error::RrubError,
    serial::{SerialBackend, SerialControlBits, SerialParity, SerialStopBits},
};

impl From<UefiParity> for SerialParity {
    fn from(value: UefiParity) -> Self {
        match value {
            UefiParity::DEFAULT => Self::Default,
            UefiParity::NONE => Self::None,
            UefiParity::EVEN => Self::Even,
            UefiParity::ODD => Self::Odd,
            UefiParity::MARK => Self::Mark,
            UefiParity::SPACE => Self::Space,
            _ => Self::Unknown(value.0),
        }
    }
}

impl From<UefiStopBits> for SerialStopBits {
    fn from(value: UefiStopBits) -> Self {
        match value {
            UefiStopBits::DEFAULT => Self::Default,
            UefiStopBits::ONE => Self::One,
            UefiStopBits::ONE_FIVE => Self::OneFive,
            UefiStopBits::TWO => Self::Two,
            _ => Self::Unknown(value.0),
        }
    }
}

impl From<SerialParity> for UefiParity {
    fn from(value: SerialParity) -> Self {
        match value {
            SerialParity::Default => Self::DEFAULT,
            SerialParity::None => Self::NONE,
            SerialParity::Even => Self::EVEN,
            SerialParity::Odd => Self::ODD,
            SerialParity::Mark => Self::MARK,
            SerialParity::Space => Self::SPACE,
            SerialParity::Unknown(e) => Self(e),
        }
    }
}

impl From<SerialStopBits> for UefiStopBits {
    fn from(value: SerialStopBits) -> Self {
        match value {
            SerialStopBits::Default => Self::DEFAULT,
            SerialStopBits::One => Self::ONE,
            SerialStopBits::OneFive => Self::ONE_FIVE,
            SerialStopBits::Two => Self::TWO,
            SerialStopBits::Unknown(e) => Self(e),
        }
    }
}

impl SerialBackend for Serial {
    fn control_bits(&self) -> Result<SerialControlBits, RrubError> {
        return Ok(SerialControlBits::from_bits_retain(
            self.get_control_bits()?.bits(),
        ));
    }

    fn timeout(&self) -> u32 {
        self.io_mode().timeout
    }

    fn set_timeout(&mut self, timeout: u32) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout,
            baud_rate: 0,
            receive_fifo_depth: 0,
            data_bits: 0,
            parity: UefiParity::DEFAULT,
            stop_bits: UefiStopBits::DEFAULT,
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn baud_rate(&self) -> u64 {
        self.io_mode().baud_rate
    }

    fn set_baud_rate(&mut self, baud_rate: u64) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout: 0,
            baud_rate,
            receive_fifo_depth: 0,
            data_bits: 0,
            parity: UefiParity::DEFAULT,
            stop_bits: UefiStopBits::DEFAULT,
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn receive_fifo_depth(&self) -> u32 {
        self.io_mode().receive_fifo_depth
    }

    fn set_receive_fifo_depth(&mut self, receive_fifo_depth: u32) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout: 0,
            baud_rate: 0,
            receive_fifo_depth,
            data_bits: 0,
            parity: UefiParity::DEFAULT,
            stop_bits: UefiStopBits::DEFAULT,
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn data_bits(&self) -> u32 {
        self.io_mode().data_bits
    }

    fn set_data_bits(&mut self, data_bits: u32) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout: 0,
            baud_rate: 0,
            receive_fifo_depth: 0,
            data_bits,
            parity: UefiParity::DEFAULT,
            stop_bits: UefiStopBits::DEFAULT,
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn parity(&self) -> SerialParity {
        SerialParity::from(self.io_mode().parity)
    }

    fn set_parity(&mut self, parity: SerialParity) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout: 0,
            baud_rate: 0,
            receive_fifo_depth: 0,
            data_bits: 0,
            parity: parity.into(),
            stop_bits: UefiStopBits::DEFAULT,
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn stop_bits(&self) -> SerialStopBits {
        SerialStopBits::from(self.io_mode().stop_bits)
    }

    fn set_stop_bits(&mut self, stop_bits: SerialStopBits) -> Result<(), RrubError> {
        let attr = IoMode {
            control_mask: UefiControlBits::default(),
            timeout: 0,
            baud_rate: 0,
            receive_fifo_depth: 0,
            data_bits: 0,
            parity: UefiParity::DEFAULT,
            stop_bits: stop_bits.into(),
        };

        return Ok(self.set_attributes(&attr)?);
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), usize> {
        match self.read(data) {
            Ok(t) => Ok(t),
            Err(e) => Err(*e.data()),
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<(), usize> {
        match self.write(data) {
            Ok(t) => Ok(t),
            Err(e) => Err(*e.data()),
        }
    }
}
