//! UPS Variable-Length Value encoding/decoding

use stitchr_core::{PatchError, Result};

/// Decode a UPS VLV from bytes
/// Returns (value, bytes_read)
pub fn decode(data: &[u8]) -> Result<(u64, usize)> {
    let mut value: u64 = 0;
    let mut shift: u64 = 1;
    let mut bytes_read = 0;

    for &byte in data {
        bytes_read += 1;

        let delta = ((byte & 0x7f) as u64)
            .checked_mul(shift)
            .ok_or(PatchError::InvalidFormat("Varint overflow".to_string()))?;

        value = value
            .checked_add(delta)
            .ok_or(PatchError::InvalidFormat("Varint overflow".to_string()))?;

        if (byte & 0x80) != 0 {
            return Ok((value, bytes_read));
        }

        shift = shift
            .checked_shl(7)
            .ok_or(PatchError::InvalidFormat("Varint overflow".to_string()))?;

        value = value
            .checked_add(shift)
            .ok_or(PatchError::InvalidFormat("Varint overflow".to_string()))?;
    }

    Err(PatchError::InvalidFormat(
        "Incomplete VLV encoding".to_string(),
    ))
}
