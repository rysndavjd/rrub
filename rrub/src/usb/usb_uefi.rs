use alloc::format;
use uefi_raw::protocol::usb::{host_controller::{HostControllerState, ResetAttributes, Speed, Usb2HostControllerProtocol
    }, DeviceRequest};
use uefi::{println, Identify, Result as UefiResult, Status, StatusExt};
use uefi::proto::unsafe_protocol;
use uefi::boot;

use crate::error::RrubError;

#[derive(Debug)]
#[repr(transparent)]
#[unsafe_protocol(Usb2HostControllerProtocol::GUID)]
pub struct Usb2HostController(Usb2HostControllerProtocol);

impl Usb2HostController {
    pub fn get_capability(&self) -> UefiResult<(Speed, u8, bool)> {
        let mut speed = Speed::FULL;
        let mut port: u8 = 0;
        let mut is_64_bit_capable: u8 = 0;

        unsafe { (self.0.get_capability)(&self.0, &mut speed, &mut port, &mut is_64_bit_capable) }
            .to_result_with_val(|| {
                return (speed, port, is_64_bit_capable == 1);
            })
    }

    pub fn reset(&mut self, attributes: ResetAttributes) -> UefiResult<()> {
        unsafe { (self.0.reset)(&mut self.0, attributes) }
            .to_result()
    }

    pub fn get_state(&mut self) -> UefiResult<HostControllerState> {
        let mut state = HostControllerState::HALT;

        unsafe { (self.0.get_state)(&mut self.0, &mut state) }
            .to_result_with_val(|| {
                return state;
            })
    }

    pub fn set_state(&mut self, state: HostControllerState) -> UefiResult<()> {
        unsafe { (self.0.set_state)(&mut self.0, state) }
            .to_result()
    }

}

pub fn usb_test() -> Result<(), RrubError> {
    let usb_handle = boot::get_handle_for_protocol::<Usb2HostController>().unwrap();
    let mut usb = boot::open_protocol_exclusive::<Usb2HostController>(usb_handle).unwrap();

    let result = usb.get_capability().unwrap();

    let format = format!("{:?}", result);

    println!("{}", format);
    return Ok(());
}