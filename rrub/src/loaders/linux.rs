use core::ffi::c_char;

use bitflags::bitflags;
use zerocopy::{
    FromBytes, Immutable, IntoBytes, LittleEndian, Unaligned,
    byteorder::{U16, U32, U64},
};

pub mod common;
pub mod x86;
