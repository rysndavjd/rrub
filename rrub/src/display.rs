mod bios_vesa;
mod efi_fb;

#[cfg(feature = "uefi")]
pub use efi_fb::UefiDisplay as Display;

#[cfg(feature = "bios")]
pub use bios_vesa::VesaDisplay as Display;

