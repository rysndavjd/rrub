use zerocopy::{
    FromBytes, Immutable, IntoBytes, KnownLayout, NativeEndian, U16, U32, U64, Unaligned,
};

pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];

pub const ELF_CLASS_32: u8 = 1;
pub const ELF_CLASS_64: u8 = 2;

pub const ELF_DATA_LITTLE_ENDIAN: u8 = 1;
pub const ELF_DATA_BIG_ENDIAN: u8 = 2;

enum ElfErrors {
    InvalidMagic,
}

#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct Elf32Header {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    _pad: [u8; 7],
    pub e_type: U16<NativeEndian>,
    pub e_cpu: U16<NativeEndian>,
    pub e_version: U32<NativeEndian>,
    pub e_entry: U32<NativeEndian>,
    pub e_phoff: U32<NativeEndian>,
    pub e_shoff: U32<NativeEndian>,
    pub e_flags: U32<NativeEndian>,
    pub e_ehsize: U16<NativeEndian>,
    pub e_phentsize: U16<NativeEndian>,
    pub e_phnum: U16<NativeEndian>,
    pub e_shentsize: U16<NativeEndian>,
    pub e_shnum: U16<NativeEndian>,
    pub e_shstrndx: U16<NativeEndian>,
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
            e_type: U16::ZERO,
            e_cpu: U16::ZERO,
            e_version: U32::ZERO,
            e_entry: U32::ZERO,
            e_phoff: U32::ZERO,
            e_shoff: U32::ZERO,
            e_flags: U32::ZERO,
            e_ehsize: U16::ZERO,
            e_phentsize: U16::ZERO,
            e_phnum: U16::ZERO,
            e_shentsize: U16::ZERO,
            e_shnum: U16::ZERO,
            e_shstrndx: U16::ZERO,
        }
    }

    pub fn verify() {}
}

#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct Elf32ProgramHeader {
    pub p_type: U32<NativeEndian>,
    pub p_offset: U32<NativeEndian>,
    pub p_vaddr: U32<NativeEndian>,
    pub p_paddr: U32<NativeEndian>,
    pub p_fsize: U32<NativeEndian>,
    pub p_msize: U32<NativeEndian>,
    pub p_flags: U32<NativeEndian>,
    pub p_align: U32<NativeEndian>,
}

#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct Elf64Header {
    pub mag: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    _pad: [u8; 7],
    pub e_type: U16<NativeEndian>,
    pub e_cpu: U16<NativeEndian>,
    pub e_version: U32<NativeEndian>,
    pub e_entry: U64<NativeEndian>,
    pub e_phoff: U64<NativeEndian>,
    pub e_shoff: U64<NativeEndian>,
    pub e_flags: U32<NativeEndian>,
    pub e_ehsize: U16<NativeEndian>,
    pub e_phentsize: U16<NativeEndian>,
    pub e_phnum: U16<NativeEndian>,
    pub e_shentsize: U16<NativeEndian>,
    pub e_shnum: U16<NativeEndian>,
    pub e_shstrndx: U16<NativeEndian>,
}

impl Elf64Header {
    pub fn empty() -> Elf64Header {
        Elf64Header {
            mag: [0u8; 4],
            class: 0,
            data: 0,
            version: 0,
            os_abi: 0,
            abi_version: 0,
            _pad: [0u8; 7],
            e_type: U16::ZERO,
            e_cpu: U16::ZERO,
            e_version: U32::ZERO,
            e_entry: U64::ZERO,
            e_phoff: U64::ZERO,
            e_shoff: U64::ZERO,
            e_flags: U32::ZERO,
            e_ehsize: U16::ZERO,
            e_phentsize: U16::ZERO,
            e_phnum: U16::ZERO,
            e_shentsize: U16::ZERO,
            e_shnum: U16::ZERO,
            e_shstrndx: U16::ZERO,
        }
    }

    pub fn verify() {}
}

#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct Elf64ProgramHeader {
    pub p_type: U32<NativeEndian>,
    pub p_flags: U32<NativeEndian>,
    pub p_offset: U64<NativeEndian>,
    pub p_vaddr: U64<NativeEndian>,
    pub p_paddr: U64<NativeEndian>,
    pub p_filesz: U64<NativeEndian>,
    pub p_memsz: U64<NativeEndian>,
    pub p_align: U64<NativeEndian>,
}
