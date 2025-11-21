#[cfg(feature = "uefi")]
use uefi::mem::memory_map::{MemoryDescriptor, MemoryType};
use zerocopy::{FromBytes, Immutable, IntoBytes, LittleEndian, U32, U64, Unaligned};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum E820Type {
    Ram = 1,
    Reserved = 2,
    Acpi = 3,
    Nvs = 4,
    Unusable = 5,
    Pmem = 7,
    Pram = 12,
    SoftReserved = 0xefffffff,
}

#[cfg(feature = "uefi")]
impl From<MemoryType> for E820Type {
    fn from(value: MemoryType) -> Self {
        match value {
            MemoryType::CONVENTIONAL => E820Type::Ram,
            MemoryType::LOADER_CODE => E820Type::Ram,
            MemoryType::LOADER_DATA => E820Type::Ram,
            MemoryType::BOOT_SERVICES_CODE => E820Type::Ram,
            MemoryType::BOOT_SERVICES_DATA => E820Type::Ram,

            MemoryType::ACPI_RECLAIM => E820Type::Acpi,

            MemoryType::ACPI_NON_VOLATILE => E820Type::Nvs,

            MemoryType::UNUSABLE => E820Type::Unusable,

            MemoryType::PERSISTENT_MEMORY => E820Type::Pmem,

            MemoryType::RUNTIME_SERVICES_CODE => E820Type::Reserved,
            MemoryType::RUNTIME_SERVICES_DATA => E820Type::Reserved,
            MemoryType::RESERVED => E820Type::Reserved,
            MemoryType::MMIO => E820Type::Reserved,
            MemoryType::MMIO_PORT_SPACE => E820Type::Reserved,
            MemoryType::PAL_CODE => E820Type::Reserved,

            _ => E820Type::Unusable,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct E820Entry {
    pub addr: U64<LittleEndian>,
    pub size: U64<LittleEndian>,
    entry_type: U32<LittleEndian>,
}

#[cfg(feature = "uefi")]
impl From<MemoryDescriptor> for E820Entry {
    fn from(value: MemoryDescriptor) -> Self {
        Self {
            addr: U64::new(value.phys_start),
            size: U64::new(value.page_count),
            entry_type: U32::new(value.ty.0),
        }
    }
}

impl E820Entry {
    pub fn entry_type(&self) -> Option<E820Type> {
        return match self.entry_type.get() {
            1 => Some(E820Type::Ram),
            2 => Some(E820Type::Reserved),
            3 => Some(E820Type::Acpi),
            4 => Some(E820Type::Nvs),
            5 => Some(E820Type::Unusable),
            7 => Some(E820Type::Pmem),
            12 => Some(E820Type::Pram),
            0xefffffff => Some(E820Type::SoftReserved),
            _ => None,
        };
    }
}
