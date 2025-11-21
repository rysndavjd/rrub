use zerocopy::{
    FromBytes, Immutable, IntoBytes, LittleEndian, Unaligned,
    byteorder::{U16, U32, U64},
};

use crate::{
    loaders::linux::common::{ApmBiosInfo, EdidInfo, IstInfo, ScreenInfo},
    mem::e820::E820Entry,
};

pub const E820_MAX_ENTRIES_ZEROPAGE: usize = 128;
pub const EDD_MBR_SIG_MAX: usize = 16;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct SetupHeader {
    pub setup_sects: u8,
    pub root_flags: U16<LittleEndian>,
    pub syssize: U32<LittleEndian>,
    pub ram_size: U16<LittleEndian>,
    pub vid_mode: U16<LittleEndian>,
    pub root_dev: U16<LittleEndian>,
    pub boot_flag: U16<LittleEndian>,
    pub jump: U16<LittleEndian>,
    pub header: U32<LittleEndian>,
    pub version: U16<LittleEndian>,
    pub realmode_swtch: U32<LittleEndian>,
    pub start_sys_seg: U16<LittleEndian>,
    pub kernel_version: U16<LittleEndian>,
    pub type_of_loader: u8,
    pub loadflags: u8,
    pub setup_move_size: U16<LittleEndian>,
    pub code32_start: U32<LittleEndian>,
    pub ramdisk_image: U32<LittleEndian>,
    pub ramdisk_size: U32<LittleEndian>,
    pub bootsect_kludge: U32<LittleEndian>,
    pub heap_end_ptr: U16<LittleEndian>,
    pub ext_loader_ver: u8,
    pub ext_loader_type: u8,
    pub cmd_line_ptr: U32<LittleEndian>,
    pub initrd_addr_max: U32<LittleEndian>,
    pub kernel_alignment: U32<LittleEndian>,
    pub relocatable_kernel: u8,
    pub min_alignment: u8,
    pub xloadflags: U16<LittleEndian>,
    pub cmdline_size: U32<LittleEndian>,
    pub hardware_subarch: U32<LittleEndian>,
    pub hardware_subarch_data: U64<LittleEndian>,
    pub payload_offset: U32<LittleEndian>,
    pub payload_length: U32<LittleEndian>,
    pub setup_data: U64<LittleEndian>,
    pub pref_address: U64<LittleEndian>,
    pub init_size: U32<LittleEndian>,
    pub handover_offset: U32<LittleEndian>,
    pub kernel_info_offset: U32<LittleEndian>,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct SysDescTable {
    length: U16<LittleEndian>,
    table: [u8; 14],
}

impl SysDescTable {
    pub fn empty() -> SysDescTable {
        SysDescTable {
            length: U16::ZERO,
            table: [0u8; 14],
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct OlpcOfwHeader {
    ofw_magic: U32<LittleEndian>,
    ofw_version: U32<LittleEndian>,
    cif_handler: U32<LittleEndian>,
    irq_desc_table: U32<LittleEndian>,
}

impl OlpcOfwHeader {
    pub fn empty() -> OlpcOfwHeader {
        OlpcOfwHeader {
            ofw_magic: U32::ZERO,
            ofw_version: U32::ZERO,
            cif_handler: U32::ZERO,
            irq_desc_table: U32::ZERO,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct EfiInfo {
    efi_loader_signature: U32<LittleEndian>,
    efi_systab: U32<LittleEndian>,
    efi_memdesc_size: U32<LittleEndian>,
    efi_memdesc_version: U32<LittleEndian>,
    efi_memmap: U32<LittleEndian>,
    efi_memmap_size: U32<LittleEndian>,
    efi_systab_hi: U32<LittleEndian>,
    efi_memmap_hi: U32<LittleEndian>,
}

impl EfiInfo {
    pub fn empty() -> EfiInfo {
        EfiInfo {
            efi_loader_signature: U32::ZERO,
            efi_systab: U32::ZERO,
            efi_memdesc_size: U32::ZERO,
            efi_memdesc_version: U32::ZERO,
            efi_memmap: U32::ZERO,
            efi_memmap_size: U32::ZERO,
            efi_systab_hi: U32::ZERO,
            efi_memmap_hi: U32::ZERO,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct Zeropage {
    pub screen_info: ScreenInfo,
    pub apm_bios_info: ApmBiosInfo,
    pub _pad2: [u8; 4],
    pub tboot_addr: U64<LittleEndian>,
    pub ist_info: IstInfo,
    pub acpi_rsdp_addr: U64<LittleEndian>,
    pub _pad3: [u8; 8],
    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],
    pub sys_desc_table: SysDescTable,
    pub olpc_ofw_header: OlpcOfwHeader,
    pub ext_ramdisk_image: U32<LittleEndian>,
    pub ext_ramdisk_size: U32<LittleEndian>,
    pub ext_cmd_line_ptr: U32<LittleEndian>,
    pub _pad4: [u8; 112],
    pub cc_blob_address: U32<LittleEndian>,
    pub edid_info: EdidInfo,
    pub efi_info: EfiInfo,
    pub alt_mem_k: U32<LittleEndian>,
    pub scratch: U32<LittleEndian>,
    pub e820_entries: u8,
    pub eddbuf_entries: u8,
    pub edd_mbr_sig_buf_entries: u8,
    pub kbd_status: u8,
    pub secure_boot: u8,
    pub _pad5: [u8; 2],
    pub sentinel: u8,
    pub _pad6: u8,
    pub hdr: SetupHeader,
    pub _pad7: [u8; 0x290 - 0x1f1 - size_of::<SetupHeader>()],
    pub edd_mbr_sig_buffer: [U32<LittleEndian>; EDD_MBR_SIG_MAX],
    pub e820_table: [E820Entry; E820_MAX_ENTRIES_ZEROPAGE],
    pub _pad8: [u8; 816],
}
