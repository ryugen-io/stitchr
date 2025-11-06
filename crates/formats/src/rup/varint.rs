//! RUP Variable-Length Value (VLV) encoding

use rom_patcher_core::{PatchError, Result};

/// Decode a RUP VLV value
///
/// RUP VLV format:
/// - First byte: number of following bytes
/// - Following bytes: little-endian value
///
/// Returns (decoded_value, total_bytes_consumed)
pub fn decode_vlv(data: &[u8]) -> Result<(u64, usize)> {
    if data.is_empty() {
        return Err(PatchError::UnexpectedEof("VLV header".to_string()));
    }

    let num_bytes = data[0] as usize;

    if num_bytes == 0 {
        return Ok((0, 1));
    }

    if data.len() < 1 + num_bytes {
        return Err(PatchError::UnexpectedEof(format!(
            "VLV data (expected {} bytes)",
            num_bytes
        )));
    }

    let mut value: u64 = 0;
    for i in 0..num_bytes {
        value |= (data[1 + i] as u64) << (i * 8);
    }

    Ok((value, 1 + num_bytes))
}
