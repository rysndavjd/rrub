use core::ptr::NonNull;

use uefi::{
    boot::{
        AllocateType, MemoryAttribute, MemoryType, OpenProtocolParams, allocate_pages, free_pages,
        get_handle_for_protocol, image_handle, open_protocol, open_protocol_exclusive,
    },
    proto::security::MemoryProtection,
};

use crate::{
    error::RrubError,
    mem::{MemAttr, MemoryBackend},
};

impl From<MemoryAttribute> for MemAttr {
    fn from(value: MemoryAttribute) -> Self {
        let mut attr = MemAttr::all();

        if value.contains(MemoryAttribute::READ_PROTECT) {
            attr.remove(MemAttr::Read);
        }

        if value.contains(MemoryAttribute::READ_ONLY) {
            attr.remove(MemAttr::Write);
        }

        if value.contains(MemoryAttribute::EXECUTE_PROTECT) {
            attr.remove(MemAttr::Execute);
        }

        return attr;
    }
}

#[derive(Debug)]
pub struct UefiMemory {}

impl<T> MemoryBackend<T> for UefiMemory {
    fn allocate(addr: usize, page_count: usize) -> Result<NonNull<T>, RrubError> {
        assert!((addr & 0xFFF) == 0, "Addr not aligned to 4096 bytes");
        let ptr = allocate_pages(
            AllocateType::Address(addr as u64),
            MemoryType::LOADER_DATA,
            page_count,
        )?;

        return Ok(unsafe { NonNull::new_unchecked(ptr.as_ptr() as *mut T) });
    }

    unsafe fn deallocate(ptr: NonNull<T>, page_count: usize) -> Result<(), RrubError> {
        unsafe {
            free_pages(NonNull::new_unchecked(ptr.as_ptr() as *mut u8), page_count)?;
        }
        return Ok(());
    }

    fn get_mem_attrs(addr: usize, page_count: usize) -> Result<MemAttr, RrubError> {
        let handle = get_handle_for_protocol::<MemoryProtection>()?;
        let mem_protect = unsafe {
            open_protocol::<MemoryProtection>(OpenProtocolParams {
                handle,
                agent: image_handle(),
                controller: None,
            },
        )?
        };

        let uefi_attr = mem_protect
            .get_memory_attributes((addr as u64)..((addr + (page_count * 4096)) as u64))?;

        return Ok(MemAttr::from(uefi_attr));
    }

    unsafe fn update_mem_attrs(
        addr: usize,
        page_count: usize,
        new_attr: super::MemAttr,
        clear_attr: super::MemAttr,
    ) -> Result<(), RrubError> {
    }
}
