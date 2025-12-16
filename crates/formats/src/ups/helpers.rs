//! UPS helper functions

use super::constants::*;
use super::varint;
use crc32fast;
use stitchr_core::{PatchError, Result};

/// Parse UPS header and return (input_size, output_size, data_offset)
pub fn parse_header(patch: &[u8]) -> Result<(u64, u64, usize)> {
    let mut offset = MAGIC_SIZE;

    let (input_size, bytes_read) = varint::decode(&patch[offset..])?;
    offset += bytes_read;

    let (output_size, bytes_read) = varint::decode(&patch[offset..])?;
    offset += bytes_read;

    Ok((input_size, output_size, offset))
}

/// Generic CRC32 validation
fn validate_crc32(data: &[u8], patch: &[u8], offset: usize) -> Result<()> {
    if offset + 4 > patch.len() {
        return Err(PatchError::InvalidFormat(
            "Patch too small for CRC32".to_string(),
        ));
    }

    let expected = u32::from_le_bytes([
        patch[offset],
        patch[offset + 1],
        patch[offset + 2],
        patch[offset + 3],
    ]);

    let actual = crc32fast::hash(data);
    if expected != actual {
        return Err(PatchError::ChecksumMismatch { expected, actual });
    }
    Ok(())
}

/// Validate input ROM CRC32
pub fn validate_input_crc(rom: &[u8], patch: &[u8]) -> Result<()> {
    validate_crc32(rom, patch, patch.len() - FOOTER_SIZE)
}

/// Validate output ROM CRC32
pub fn validate_output_crc(target: &[u8], patch: &[u8]) -> Result<()> {
    validate_crc32(target, patch, patch.len() - 8)
}

/// Validate patch integrity CRC32
pub fn validate_patch_crc(patch: &[u8]) -> Result<()> {
    if patch.len() < FOOTER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch too small for footer".to_string(),
        ));
    }

    let expected = u32::from_le_bytes([
        patch[patch.len() - 4],
        patch[patch.len() - 3],
        patch[patch.len() - 2],
        patch[patch.len() - 1],
    ]);

    let actual = crc32fast::hash(&patch[..patch.len() - 4]);
    if expected != actual {
        return Err(PatchError::ChecksumMismatch { expected, actual });
    }
    Ok(())
}
