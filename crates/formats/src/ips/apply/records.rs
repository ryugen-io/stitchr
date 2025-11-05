//! IPS record handlers (RLE and normal)

use crate::ips::io::read_u16_be;
use rom_patcher_core::{PatchError, Result};

/// Apply a single IPS record (normal or RLE)
pub fn apply_record(
    rom: &mut Vec<u8>,
    patch: &[u8],
    mut offset: usize,
    record_offset: usize,
) -> Result<usize> {
    if offset + 2 > patch.len() {
        return Err(PatchError::CorruptedData);
    }

    let size = read_u16_be(&patch[offset..offset + 2]);
    offset += 2;

    if size == 0 {
        apply_rle_record(rom, patch, offset, record_offset)
    } else {
        apply_normal_record(rom, patch, offset, record_offset, size as usize)
    }
}

/// Apply RLE (Run-Length Encoded) record.
///
/// RLE records are identified by size == 0. They consist of:
/// - Count (2 bytes, big-endian): number of times to repeat the value
/// - Value (1 byte): the byte to repeat
///
/// Example: Writing 256 copies of 0xFF at offset 0x1000
/// ```text
/// [00 10 00]  ← Offset: 0x001000
/// [00 00]     ← Size: 0 (RLE marker)
/// [01 00]     ← Count: 256
/// [FF]        ← Value: 0xFF
/// ```
///
/// This is more efficient than storing 256 bytes of 0xFF in the patch.
fn apply_rle_record(
    rom: &mut Vec<u8>,
    patch: &[u8],
    offset: usize,
    record_offset: usize,
) -> Result<usize> {
    // Ensure we have enough bytes for: count (2) + value (1)
    if offset + 3 > patch.len() {
        return Err(PatchError::CorruptedData);
    }

    // Read how many times to repeat (count)
    let rle_size = read_u16_be(&patch[offset..offset + 2]) as usize;
    // Read the byte value to repeat
    let value = patch[offset + 2];

    // Ensure ROM is large enough for this write
    ensure_rom_size(rom, record_offset + rle_size);

    // Fill the range with the repeated value
    rom[record_offset..record_offset + rle_size].fill(value);

    // Return next offset (skip count + value bytes)
    Ok(offset + 3)
}

/// Apply normal (non-RLE) record
fn apply_normal_record(
    rom: &mut Vec<u8>,
    patch: &[u8],
    offset: usize,
    record_offset: usize,
    size: usize,
) -> Result<usize> {
    if offset + size > patch.len() {
        return Err(PatchError::CorruptedData);
    }

    ensure_rom_size(rom, record_offset + size);
    rom[record_offset..record_offset + size].copy_from_slice(&patch[offset..offset + size]);

    Ok(offset + size)
}

/// Ensure ROM is large enough for the operation
#[inline]
fn ensure_rom_size(rom: &mut Vec<u8>, required_size: usize) {
    if rom.len() < required_size {
        rom.resize(required_size, 0);
    }
}
