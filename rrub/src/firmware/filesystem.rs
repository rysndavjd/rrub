use alloc::vec::Vec;

use serde::Deserialize;
use uuid::Uuid as RealUuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
pub struct VolumeId32([u8; 4]);

impl VolumeId32 {
    pub fn nil() -> Self {
        VolumeId32([0u8; 4])
    }

    pub fn max() -> Self {
        VolumeId32([0xFFu8; 4])
    }

    pub fn new(value: [u8; 4]) -> Self {
        VolumeId32(value)
    }

    pub fn from_u32_le(value: u32) -> VolumeId32 {
        VolumeId32(value.to_le_bytes())
    }

    pub fn from_u32_be(value: u32) -> VolumeId32 {
        VolumeId32(value.to_be_bytes())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
pub struct VolumeId64([u8; 8]);

impl VolumeId64 {
    pub fn nil() -> Self {
        VolumeId64([0u8; 8])
    }

    pub fn max() -> Self {
        VolumeId64([0xFFu8; 8])
    }

    pub fn new(value: [u8; 8]) -> Self {
        VolumeId64(value)
    }

    pub fn from_u64_le(value: u64) -> VolumeId64 {
        VolumeId64(value.to_le_bytes())
    }

    pub fn from_u64_be(value: u64) -> VolumeId64 {
        VolumeId64(value.to_be_bytes())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
pub enum Uuid {
    RealUuid(RealUuid),
    VolumeId32(VolumeId32),
    VolumeId64(VolumeId64),
}

#[derive(Debug, Copy, Clone)]
pub struct Filesystem {}

#[derive(Debug, Clone)]
pub struct FilesystemsList {
    filesystems: Vec<(Uuid, Filesystem)>,
}
