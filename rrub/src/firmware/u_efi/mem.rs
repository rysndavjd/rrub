use core::ptr::NonNull;

use uefi::{
    boot::{
        AllocateType, MemoryAttribute, MemoryType, OpenProtocolAttributes, OpenProtocolParams,
        allocate_pages, free_pages, get_handle_for_protocol, image_handle, open_protocol,
    },
    proto::security::MemoryProtection,
};

use crate::error::RrubError;

