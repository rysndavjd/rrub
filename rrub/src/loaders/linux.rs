//#[cfg(all(target_arch = "x86_64", target_os = "uefi"))]
use zerocopy::{FromBytes, IntoBytes, Unaligned, Immutable, 
    byteorder::little_endian::{U64, U32, U16}};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VideoType {
    Mda = 0x10,
    Cga = 0x11,
    Egam = 0x20,
    Egac = 0x21,
    Vgac = 0x22,
    Vlfb = 0x23,
    PicaS3 = 0x30,
    MipsG364 = 0x31,
    Sgi = 0x33,
    Tgac = 0x40,
    Sun = 0x50,
    SunPci = 0x51,
    Pmac = 0x60,
    Efi = 0x70,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
struct E820Entry {
    addr: U64,
    size: U64,
    entry_type: U32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct LinuxKernelParams {
    // screen_info
    orig_x: u8,
    orig_y: u8,
    ext_mem_k: U16,
    orig_video_page: U16,
    orig_video_mode: u8, // VideoType
    orig_video_cols: u8,
    flags: u8,
    unused2: u8,
    orig_video_ega_bx: U16,
    unused3: U16,
    orig_video_lines: u8,
    orig_video_is_vga: u8,
    orig_video_points: U16,

    lfb_width: U16,
    lfb_height: U16,
    lfb_depth: U16,
    lfb_base: U32,
    lfb_size: U32,
    cl_magic: U16,
    cl_offset: U16,
    lfb_linelength: U16,
    red_size: u8,
    red_pos: u8,
    green_size: u8,
    green_pos: u8,
    blue_size: u8,
    blue_pos: u8,
    rsvd_size: u8,
    rsvd_pos: u8,
    vesapm_seg: U16,
    vesapm_off: U16,
    pages: U16,
    vesa_attributes: U16,
    capabilities: U32,
    ext_lfb_base: U32,
    screen_info_reserved: [u8; 2],
    
    // apm_bios_info
    apm_version: U16,
    apm_cseg: U16,
    apm_offset: U32,
    apm_cseg_16: U16,
    apm_dseg: U16,
    apm_flags: U16,
    apm_cseg_len: U16,
    apm_cseg_16_len: U16,
    apm_dseg_len: U16,

    padding2: [u8; 4],
    tboot_addr: U64,

    // ist_info
    ist_info_signature: U32,
    ist_info_command: U32,
    ist_info_event: U32,
    ist_info_perf_level: U32,

    acpi_rsdp_addr: U64,
    padding3: [u8; 8],
    hd0_info: [u8; 16], /* obsolete */
    hd1_info: [u8; 16], /* obsolete */

    // sys_desc_table /* obsolete */
    sys_desc_table_length: U16, 
    sys_desc_table: [u8; 14],

    // olpc_ofw_header
    ofw_magic: U32,
    ofw_version: U32,
    cif_handler: U32,
    irq_desc_table: U32,

    ext_ramdisk_image: U32,
    ext_ramdisk_size: U32,
    ext_cmd_line_ptr: U32,
    padding4: [u8; 112],
    cc_blob_address: U32,

    // edid_info
    edid_info_dummy: [u8; 128],

    // efi_info
    efi_loader_signature: U32,
    efi_systab: U32,
    efi_memdesc_size: U32,
    efi_memdesc_version: U32,
    efi_memmap: U32,
    efi_memmap_size: U32,
    efi_systab_hi: U32,
    efi_memmap_hi: U32,

    alt_mem_k: U32,
    scratch: U32,
    e820_entries: u8,
    eddbuf_entries: u8,
    edd_mbr_sig_buf_entries: u8,
    kbd_status: u8,
    secure_boot: u8,
    padding5: [u8; 2],

    sentinel: u8,
    padding6: u8,
    
    // setup_header
    setup_sects: u8,
    root_flags: U16,
    syssize: U32,
    ram_size: U16,
    vid_mode: U16,
    root_dev: U16,
    boot_flag: U16,
    jump: U16,
    header: U32,
    version: U16,
    realmode_swtch: U32,
    start_sys_seg: U16,
    kernel_version: U16,
    type_of_loader: u8,
    loadflags: u8,
    setup_move_size: U16,
    code32_start: U32,
    ramdisk_image: U32,
    ramdisk_size: U32,
    bootsect_kludge: U32,
    heap_end_ptr: U16,
    ext_loader_ver: u8,
    ext_loader_type: u8,
    cmd_line_ptr: U32,
    initrd_addr_max: U32,
    kernel_alignment: U32,
    relocatable_kernel: u8,
    min_alignment: u8,
    xloadflags: U16,
    cmdline_size: U32,
    hardware_subarch: U32,
    hardware_subarch_data: U64,
    payload_offset: U32,
    payload_length: U32,
    setup_data: U64,
    pref_address: U64,
    init_size: U32,
    handover_offset: U32,
    kernel_info_offset: U32,

    padding7: [u8; 40],
    edd_mbr_sig_buffer: [U32; 16],
    e820_map: [E820Entry; (0x400 - 0x2d0) / 20],
}