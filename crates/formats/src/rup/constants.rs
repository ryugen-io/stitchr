//! RUP format constants

/// Magic bytes: "NINJA2"
pub const MAGIC: &[u8; 6] = b"NINJA2";
pub const MAGIC_SIZE: usize = 6;

/// Header size (metadata section)
pub const HEADER_SIZE: usize = 0x800; // 2048 bytes

/// Command bytes
pub const COMMAND_END: u8 = 0x00;
pub const COMMAND_OPEN_NEW_FILE: u8 = 0x01;
pub const COMMAND_XOR_RECORD: u8 = 0x02;

/// Overflow modes
pub const OVERFLOW_APPEND: u8 = b'A';
pub const OVERFLOW_MINIFY: u8 = b'M';

/// Metadata field offsets and sizes
pub const OFFSET_TEXT_ENCODING: usize = 0x06;
pub const OFFSET_AUTHOR: usize = 0x07;
pub const SIZE_AUTHOR: usize = 84;
pub const OFFSET_VERSION: usize = 0x5B;
pub const SIZE_VERSION: usize = 11;
pub const OFFSET_TITLE: usize = 0x66;
pub const SIZE_TITLE: usize = 256;
pub const OFFSET_GENRE: usize = 0x166;
pub const SIZE_GENRE: usize = 48;
pub const OFFSET_LANGUAGE: usize = 0x196;
pub const SIZE_LANGUAGE: usize = 48;
pub const OFFSET_DATE: usize = 0x1C6;
pub const SIZE_DATE: usize = 8;
pub const OFFSET_WEB: usize = 0x1CE;
pub const SIZE_WEB: usize = 512;
pub const OFFSET_DESCRIPTION: usize = 0x3CE;
pub const SIZE_DESCRIPTION: usize = 1074;

/// ROM type identifiers
pub const ROM_TYPE_RAW: u8 = 0;
pub const ROM_TYPE_NES: u8 = 1;
pub const ROM_TYPE_FDS: u8 = 2;
pub const ROM_TYPE_SNES: u8 = 3;
pub const ROM_TYPE_N64: u8 = 4;
pub const ROM_TYPE_GB: u8 = 5;
pub const ROM_TYPE_SMS: u8 = 6;
pub const ROM_TYPE_MEGA: u8 = 7;
pub const ROM_TYPE_PCE: u8 = 8;
pub const ROM_TYPE_LYNX: u8 = 9;

/// ROM type names
pub const ROM_TYPE_NAMES: &[&str] = &[
    "raw", "nes", "fds", "snes", "n64", "gb", "sms", "mega", "pce", "lynx",
];
