use uefi::boot::{MemoryDescriptor, MemoryType as UefiMemoryType};

use crate::firmware::memory::{MemoryRegion, MemoryType, PAGE_SIZE};

impl From<&MemoryDescriptor> for MemoryRegion {
    fn from(value: &MemoryDescriptor) -> Self {
        let mem_type = match value.ty {
            UefiMemoryType::CONVENTIONAL => MemoryType::Available,
            UefiMemoryType::ACPI_RECLAIM => MemoryType::Acpi,
            UefiMemoryType::ACPI_NON_VOLATILE => MemoryType::Nvs,
            UefiMemoryType::UNUSABLE => MemoryType::Unusable,
            _ => MemoryType::Reserved,
        };

        Self {
            start: value.phys_start,
            size: value.page_count * PAGE_SIZE as u64,
            region_type: mem_type,
        }
    }
}
