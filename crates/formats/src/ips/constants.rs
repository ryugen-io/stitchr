//! IPS format constants

pub(super) const HEADER: &[u8] = b"PATCH";
pub(super) const EOF_MARKER: u32 = 0x454F46;
pub const MAX_ROM_SIZE: usize = 16 * 1024 * 1024;
pub const MAX_RECORD_SIZE: usize = 65535;
