use zerocopy::{
    FromBytes, Immutable, IntoBytes, NativeEndian, Unaligned,
    byteorder::{U16, U32},
};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct ScreenInfo {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: U16<NativeEndian>,
    pub orig_video_page: U16<NativeEndian>,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: U16<NativeEndian>,
    pub unused3: [u8; 2],
    pub orig_video_lines: u8,
    pub orig_video_is_vga: u8,
    pub orig_video_points: U16<NativeEndian>,

    pub lfb_width: U16<NativeEndian>,
    pub lfb_height: U16<NativeEndian>,
    pub lfb_depth: U16<NativeEndian>,
    pub lfb_base: U32<NativeEndian>,
    pub lfb_size: U32<NativeEndian>,
    pub cl_magic: U16<NativeEndian>,
    pub cl_offset: U16<NativeEndian>,
    pub lfb_linelength: U16<NativeEndian>,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: U16<NativeEndian>,
    pub vesapm_off: U16<NativeEndian>,
    pub pages: U16<NativeEndian>,
    pub vesa_attributes: U16<NativeEndian>,
    pub capabilities: U32<NativeEndian>,
    pub ext_lfb_base: U32<NativeEndian>,
    pub _reserved: [u8; 2],
}

impl ScreenInfo {
    pub fn empty() -> ScreenInfo {
        ScreenInfo {
            orig_x: 0,
            orig_y: 0,
            ext_mem_k: U16::ZERO,
            orig_video_page: U16::ZERO,
            orig_video_mode: 0,
            orig_video_cols: 0,
            flags: 0,
            unused2: 0,
            orig_video_ega_bx: U16::ZERO,
            unused3: [0u8; 2],
            orig_video_lines: 0,
            orig_video_is_vga: 0,
            orig_video_points: U16::ZERO,
            lfb_width: U16::ZERO,
            lfb_height: U16::ZERO,
            lfb_depth: U16::ZERO,
            lfb_base: U32::ZERO,
            lfb_size: U32::ZERO,
            cl_magic: U16::ZERO,
            cl_offset: U16::ZERO,
            lfb_linelength: U16::ZERO,
            red_size: 0,
            red_pos: 0,
            green_size: 0,
            green_pos: 0,
            blue_size: 0,
            blue_pos: 0,
            rsvd_size: 0,
            rsvd_pos: 0,
            vesapm_seg: U16::ZERO,
            vesapm_off: U16::ZERO,
            pages: U16::ZERO,
            vesa_attributes: U16::ZERO,
            capabilities: U32::ZERO,
            ext_lfb_base: U32::ZERO,
            _reserved: [0u8; 2],
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct ApmBiosInfo {
    version: U16<NativeEndian>,
    cseg: U16<NativeEndian>,
    offset: U32<NativeEndian>,
    cseg_16: U16<NativeEndian>,
    dseg: U16<NativeEndian>,
    flags: U16<NativeEndian>,
    cseg_len: U16<NativeEndian>,
    cseg_16_len: U16<NativeEndian>,
    dseg_len: U16<NativeEndian>,
}

impl ApmBiosInfo {
    pub fn empty() -> ApmBiosInfo {
        ApmBiosInfo {
            version: U16::ZERO,
            cseg: U16::ZERO,
            offset: U32::ZERO,
            cseg_16: U16::ZERO,
            dseg: U16::ZERO,
            flags: U16::ZERO,
            cseg_len: U16::ZERO,
            cseg_16_len: U16::ZERO,
            dseg_len: U16::ZERO,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct EdidInfo {
    dummy: [u8; 128],
}

impl EdidInfo {
    pub fn empty() -> EdidInfo {
        EdidInfo { dummy: [0u8; 128] }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, FromBytes, IntoBytes, Unaligned, Immutable)]
pub struct IstInfo {
    signature: U32<NativeEndian>,
    command: U32<NativeEndian>,
    event: U32<NativeEndian>,
    perf_level: U32<NativeEndian>,
}

impl IstInfo {
    pub fn empty() -> IstInfo {
        IstInfo {
            signature: U32::ZERO,
            command: U32::ZERO,
            event: U32::ZERO,
            perf_level: U32::ZERO,
        }
    }
}
