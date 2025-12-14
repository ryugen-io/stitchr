//! xdelta constants (VCDIFF)

/// VCDIFF header (V | 0x80, C | 0x80, D | 0x80)
pub const VCDIFF_HEADER: &[u8] = &[0xd6, 0xc3, 0xc4];

// Header Indicator
pub const VCD_DECOMPRESS: u8 = 0x01;
pub const VCD_CODETABLE: u8 = 0x02;
pub const VCD_APPHEADER: u8 = 0x04;

// Window Indicator
pub const VCD_SOURCE: u8 = 0x01;
pub const VCD_TARGET: u8 = 0x02;
pub const VCD_ADLER32: u8 = 0x04;

// Instruction Types
pub const VCD_NOOP: u8 = 0;
pub const VCD_ADD: u8 = 1;
pub const VCD_RUN: u8 = 2;
pub const VCD_COPY: u8 = 3;

// Address Modes
pub const VCD_MODE_SELF: u8 = 0;
pub const VCD_MODE_HERE: u8 = 1;

/// Secondary compressor IDs
pub const VCD_NOCOMPRESS: u8 = 0;
pub const VCD_DJW: u8 = 1;
pub const VCD_LZMA: u8 = 2;
