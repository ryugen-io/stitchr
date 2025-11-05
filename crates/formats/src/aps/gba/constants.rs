pub const MAGIC: &[u8] = b"APS1";
pub const MAGIC_LEN: usize = 4;
pub const BLOCK_SIZE: usize = 0x10000; // 64KB
pub const HEADER_SIZE: usize = MAGIC_LEN + 4 + 4; // magic + source_size + target_size
pub const RECORD_HEADER_SIZE: usize = 4 + 2 + 2; // offset + source_crc16 + target_crc16
pub const RECORD_SIZE: usize = RECORD_HEADER_SIZE + BLOCK_SIZE;
pub const MIN_PATCH_SIZE: usize = HEADER_SIZE + RECORD_SIZE;
