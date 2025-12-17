use alloc::{string::String, vec::Vec};
use core::time::Duration;

use serde::Deserialize;

use crate::firmware::filesystem::Uuid;

#[derive(Deserialize)]
struct Config {
    /// Enable GUI to display bootselecter.
    enable_gui: bool,
    /// Enable recovery mode to manually enter an entry type.
    enable_recovery: bool,

    /// Disk to mount on boot to load associated boot entries.
    disk: Uuid,

    /// How long to delay display GUI before booting default entry,
    /// if GUI is disabled the default entry would be booted automatically without a delay.
    boot_delay: Duration,
    /// Default entry to boot.
    default_entry: String,
    /// List of entries to boot.
    entries: Vec<(String, EntryType)>,
}

#[derive(Deserialize)]
enum EntryType {
    EfiChainload(EfiChainloadEntry),
    Linux(LinuxEntry),
}

#[derive(Deserialize)]
struct EfiChainloadEntry {}

#[derive(Deserialize)]
struct LinuxEntry {}
