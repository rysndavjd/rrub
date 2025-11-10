pub mod serial_uefi;

use bitflags::bitflags;
use core::fmt::{Debug, Error as FmtError, Result as FmtResult, Write};

use crate::error::RrubError;

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SerialControlBits: u32 {
        const CLEAR_TO_SEND = 0x10;
        const DATA_SET_READY = 0x20;
        const RING_INDICATE = 0x40;
        const CARRIER_DETECT = 0x80;
        const INPUT_BUFFER_EMPTY = 0x100;
        const OUTPUT_BUFFER_EMPTY = 0x200;
        const DATA_TERMINAL_READY = 0x1;
        const REQUEST_TO_SEND = 0x2;
        const HARDWARE_LOOPBACK_ENABLE = 0x1000;
        const SOFTWARE_LOOPBACK_ENABLE = 0x2000;
        const HARDWARE_FLOW_CONTROL_ENABLE = 0x4000;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum SerialParity {
    #[default]
    Default,
    None,
    Even,
    Odd,
    Mark,
    Space,
    Unknown(u32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum SerialStopBits {
    #[default]
    Default,
    One,
    OneFive,
    Two,
    Unknown(u32),
}

pub struct SerialMode {
    pub control_mask: SerialControlBits,
    pub timeout: u32,
    pub baud_rate: u64,
    pub receive_fifo_depth: u32,
    pub data_bits: u32,
    pub parity: SerialParity,
    pub stop_bits: SerialStopBits,
}

pub trait SerialBackend {
    fn control_bits(&self) -> Result<SerialControlBits, RrubError>;
    fn timeout(&self) -> u32;
    fn set_timeout(&mut self, timeout: u32) -> Result<(), RrubError>;
    fn baud_rate(&self) -> u64;
    fn set_baud_rate(&mut self, baud_rate: u64) -> Result<(), RrubError>;
    fn receive_fifo_depth(&self) -> u32;
    fn set_receive_fifo_depth(&mut self, receive_fifo_depth: u32) -> Result<(), RrubError>;
    fn data_bits(&self) -> u32;
    fn set_data_bits(&mut self, data_bits: u32) -> Result<(), RrubError>;
    fn parity(&self) -> SerialParity;
    fn set_parity(&mut self, parity: SerialParity) -> Result<(), RrubError>;
    fn stop_bits(&self) -> SerialStopBits;
    fn set_stop_bits(&mut self, parity: SerialStopBits) -> Result<(), RrubError>;
    fn read(&mut self, data: &mut [u8]) -> Result<(), usize>;
    fn write(&mut self, data: &[u8]) -> Result<(), usize>;
}

pub struct GenericSerial<B: SerialBackend> {
    backend: B,
}

impl<B: SerialBackend> Write for GenericSerial<B> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        self.backend.write(s.as_bytes()).map_err(|_| FmtError)
    }
}
