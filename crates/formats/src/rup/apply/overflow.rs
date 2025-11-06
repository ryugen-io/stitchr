//! RUP overflow handling (Append/Minify modes)

use rom_patcher_core::Result;

/// Apply overflow data in Append mode ('A')
/// Used when target > source: appends XOR'd data
pub fn apply_append(rom: &mut Vec<u8>, overflow_data: &[u8], source_size: usize) -> Result<()> {
    rom.resize(source_size, 0);
    for &byte in overflow_data {
        rom.push(byte ^ 0xFF);
    }
    Ok(())
}

/// Apply overflow data in Minify mode ('M') during undo
/// Used when undoing: restores removed bytes
pub fn apply_minify_undo(
    rom: &mut Vec<u8>,
    overflow_data: &[u8],
    target_size: usize,
) -> Result<()> {
    rom.resize(target_size, 0);
    for &byte in overflow_data {
        rom.push(byte ^ 0xFF);
    }
    Ok(())
}
