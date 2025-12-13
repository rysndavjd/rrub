use alloc::{string::String, vec::Vec};

use toml::Deserializer;

struct Config {
    /// Enable framebuffer output to display bootselector 
    enable_gui: bool,
    enable_input: bool,

    default: String,
    entries: Vec<(String, EntryType)>,
}

enum EntryType {
    EfiChainload(EfiChainloadEntry),
    Linux(LinuxEntry),
}

struct EfiChainloadEntry {}

struct LinuxEntry {}
