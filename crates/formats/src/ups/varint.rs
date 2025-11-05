//! UPS Variable-Length Value encoding/decoding

use rom_patcher_core::{PatchError, Result};

/// Decode a UPS VLV from bytes
/// Returns (value, bytes_read)
pub fn decode(data: &[u8]) -> Result<(u64, usize)> {
    let mut value: u64 = 0;
    let mut shift: u64 = 1;
    let mut bytes_read = 0;

    for &byte in data {
        bytes_read += 1;

        value += (byte & 0x7f) as u64 * shift;

        if (byte & 0x80) != 0 {
            return Ok((value, bytes_read));
        }

        shift <<= 7;
        value += shift;
    }

    Err(PatchError::InvalidFormat(
        "Incomplete VLV encoding".to_string(),
    ))
}
