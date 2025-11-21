use zerocopy::{FromBytes, Immutable, IntoBytes, NativeEndian, U16, U32, U64, Unaligned};

pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

pub const ELF_CLASS_32: u8 = 1;
pub const ELF_CLASS_64: u8 = 2;

pub const ELF_DATA_LITTLE_ENDIAN: u8 = 1;
pub const ELF_DATA_BIG_ENDIAN: u8 = 2;

#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
#[repr(C, packed)]
pub struct Elf32Header {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    _pad: [u8; 7],
    e_type: U16<NativeEndian>,
    e_cpu: U16<NativeEndian>,
    e_version: [u8; 4],
    e_entry: [u8; 4],
    e_phoff: [u8; 4],
    e_shoff: [u8; 4],
    e_flags: [u8; 4],
    e_ehsize: U16<NativeEndian>,
    e_phentsize: U16<NativeEndian>,
    e_phnum: U16<NativeEndian>,
    e_shentsize: U16<NativeEndian>,
    e_shnum: U16<NativeEndian>,
    e_shstrndx: U16<NativeEndian>,
}

impl Elf32Header {
    pub fn empty() -> Elf32Header {
        Elf32Header {
            mag: [0u8; 4],
            class: 0,
            data: 0,
            version: 0,
            os_abi: 0,
            abi_version: 0,
            _pad: [0u8; 7],
            e_type: [0u8; 2],
            e_cpu: [0u8; 2],
            e_version: [0u8; 4],
            e_entry: [0u8; 4],
            e_phoff: [0u8; 4],
            e_shoff: [0u8; 4],
            e_flags: [0u8; 4],
            e_ehsize: [0u8; 2],
            e_phentsize: [0u8; 2],
            e_phnum: [0u8; 2],
            e_shentsize: [0u8; 2],
            e_shnum: [0u8; 2],
            e_shstrndx: [0u8; 2],
        }
    }

    pub fn e_type(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_type)),
            2 => Some(u16::from_be_bytes(self.e_type)),
            _ => None,
        }
    }

    pub fn e_cpu(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_cpu)),
            2 => Some(u16::from_be_bytes(self.e_cpu)),
            _ => None,
        }
    }

    pub fn e_version(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_version)),
            2 => Some(u32::from_be_bytes(self.e_version)),
            _ => None,
        }
    }

    pub fn e_entry(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_entry)),
            2 => Some(u32::from_be_bytes(self.e_entry)),
            _ => None,
        }
    }

    pub fn e_phoff(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_phoff)),
            2 => Some(u32::from_be_bytes(self.e_phoff)),
            _ => None,
        }
    }

    pub fn e_shoff(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_shoff)),
            2 => Some(u32::from_be_bytes(self.e_shoff)),
            _ => None,
        }
    }

    pub fn e_flags(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_flags)),
            2 => Some(u32::from_be_bytes(self.e_flags)),
            _ => None,
        }
    }

    pub fn e_ehsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_ehsize)),
            2 => Some(u16::from_be_bytes(self.e_ehsize)),
            _ => None,
        }
    }

    pub fn e_phentsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_phentsize)),
            2 => Some(u16::from_be_bytes(self.e_phentsize)),
            _ => None,
        }
    }

    pub fn e_phnum(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_phnum)),
            2 => Some(u16::from_be_bytes(self.e_phnum)),
            _ => None,
        }
    }

    pub fn e_shentsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shentsize)),
            2 => Some(u16::from_be_bytes(self.e_shentsize)),
            _ => None,
        }
    }

    pub fn e_shnum(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shnum)),
            2 => Some(u16::from_be_bytes(self.e_shnum)),
            _ => None,
        }
    }

    pub fn e_shstrndx(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shstrndx)),
            2 => Some(u16::from_be_bytes(self.e_shstrndx)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
pub struct Elf32ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_fsize: u32,
    pub p_msize: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
pub struct Elf64Header {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    _pad: [u8; 7],
    e_type: [u8; 2],
    e_cpu: [u8; 2],
    e_version: [u8; 4],
    e_entry: [u8; 8],
    e_phoff: [u8; 8],
    e_shoff: [u8; 8],
    e_flags: [u8; 4],
    e_ehsize: [u8; 2],
    e_phentsize: [u8; 2],
    e_phnum: [u8; 2],
    e_shentsize: [u8; 2],
    e_shnum: [u8; 2],
    e_shstrndx: [u8; 2],
}

impl Elf64Header {
    pub fn e_type(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_type)),
            2 => Some(u16::from_be_bytes(self.e_type)),
            _ => None,
        }
    }

    pub fn e_cpu(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_cpu)),
            2 => Some(u16::from_be_bytes(self.e_cpu)),
            _ => None,
        }
    }

    pub fn e_version(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_version)),
            2 => Some(u32::from_be_bytes(self.e_version)),
            _ => None,
        }
    }

    pub fn e_entry(&self) -> Option<u64> {
        match self.data {
            1 => Some(u64::from_le_bytes(self.e_entry)),
            2 => Some(u64::from_be_bytes(self.e_entry)),
            _ => None,
        }
    }

    pub fn e_phoff(&self) -> Option<u64> {
        match self.data {
            1 => Some(u64::from_le_bytes(self.e_phoff)),
            2 => Some(u64::from_be_bytes(self.e_phoff)),
            _ => None,
        }
    }

    pub fn e_shoff(&self) -> Option<u64> {
        match self.data {
            1 => Some(u64::from_le_bytes(self.e_shoff)),
            2 => Some(u64::from_be_bytes(self.e_shoff)),
            _ => None,
        }
    }

    pub fn e_flags(&self) -> Option<u32> {
        match self.data {
            1 => Some(u32::from_le_bytes(self.e_flags)),
            2 => Some(u32::from_be_bytes(self.e_flags)),
            _ => None,
        }
    }

    pub fn e_ehsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_ehsize)),
            2 => Some(u16::from_be_bytes(self.e_ehsize)),
            _ => None,
        }
    }

    pub fn e_phentsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_phentsize)),
            2 => Some(u16::from_be_bytes(self.e_phentsize)),
            _ => None,
        }
    }

    pub fn e_phnum(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_phnum)),
            2 => Some(u16::from_be_bytes(self.e_phnum)),
            _ => None,
        }
    }

    pub fn e_shentsize(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shentsize)),
            2 => Some(u16::from_be_bytes(self.e_shentsize)),
            _ => None,
        }
    }

    pub fn e_shnum(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shnum)),
            2 => Some(u16::from_be_bytes(self.e_shnum)),
            _ => None,
        }
    }

    pub fn e_shstrndx(&self) -> Option<u16> {
        match self.data {
            1 => Some(u16::from_le_bytes(self.e_shstrndx)),
            2 => Some(u16::from_be_bytes(self.e_shstrndx)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
pub struct Elf64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_fsize: u64,
    pub p_msize: u64,
    pub p_align: u64,
}
