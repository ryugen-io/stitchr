//! Test helpers for APS GBA

pub const BLOCK_SIZE: usize = 0x10000; // 64KB

pub fn make_header(source_size: u32, target_size: u32) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&source_size.to_le_bytes());
    patch.extend_from_slice(&target_size.to_le_bytes());
    patch
}
