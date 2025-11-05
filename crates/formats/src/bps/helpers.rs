//! BPS helper functions

use super::constants::*;
use super::varint;
use crc32fast;
use rom_patcher_core::{PatchError, Result};

/// Parse BPS header and return (source_size, target_size, data_offset)
pub fn parse_header(patch: &[u8]) -> Result<(u64, u64, usize)> {
    let mut offset = MAGIC_SIZE;

    let (source_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid source size".to_string()))?;
    offset += bytes_read;

    let (target_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid target size".to_string()))?;
    offset += bytes_read;

    let (metadata_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid metadata size".to_string()))?;
    offset += bytes_read + metadata_size as usize;

    Ok((source_size, target_size, offset))
}

/// Validate source ROM CRC32
pub fn validate_source_crc(rom: &[u8], patch: &[u8]) -> Result<()> {
    let source_crc_offset = patch.len() - FOOTER_SIZE;
    let expected = u32::from_le_bytes([
        patch[source_crc_offset],
        patch[source_crc_offset + 1],
        patch[source_crc_offset + 2],
        patch[source_crc_offset + 3],
    ]);

    let actual = crc32fast::hash(rom);
    if expected != actual {
        return Err(PatchError::ChecksumMismatch { expected, actual });
    }
    Ok(())
}

/// Validate target ROM CRC32
pub fn validate_target_crc(target: &[u8], patch: &[u8]) -> Result<()> {
    let target_crc_offset = patch.len() - 8;
    let expected = u32::from_le_bytes([
        patch[target_crc_offset],
        patch[target_crc_offset + 1],
        patch[target_crc_offset + 2],
        patch[target_crc_offset + 3],
    ]);

    let actual = crc32fast::hash(target);
    if expected != actual {
        return Err(PatchError::ChecksumMismatch { expected, actual });
    }
    Ok(())
}

/// Decode signed varint delta
#[inline]
pub fn decode_signed_delta(data: u64) -> i64 {
    if data & 1 != 0 {
        -((data >> 1) as i64)
    } else {
        (data >> 1) as i64
    }
}
