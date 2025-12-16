//! Variable-length integer encoding/decoding for BPS format
//!
//! BPS uses a custom varint encoding that supports arbitrary file sizes.
//! Each byte stores 7 bits of data, with the 8th bit as a continuation flag.

use stitchr_core::{PatchError, Result};

/// Decode a variable-length integer from a byte slice
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Algorithm
/// ```text
/// uint64 decode() {
///   uint64 data = 0, shift = 1;
///   while(true) {
///     uint8 x = read();
///     data += (x & 0x7f) * shift;
///     if(x & 0x80) break;
///     shift <<= 7;
///     data += shift;
///   }
///   return data;
/// }
/// ```
#[inline]
pub fn decode(data: &[u8]) -> Result<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift: u64 = 1;
    let mut bytes_read = 0;

    for &byte in data.iter() {
        bytes_read += 1;

        // Add lower 7 bits multiplied by shift
        let part = ((byte & 0x7f) as u64)
            .checked_mul(shift)
            .ok_or_else(|| PatchError::Other("Varint multiplication overflow".to_string()))?;

        result = result
            .checked_add(part)
            .ok_or_else(|| PatchError::Other("Varint overflow".to_string()))?;

        // Check termination bit
        if byte & 0x80 != 0 {
            return Ok((result, bytes_read));
        }

        // Update shift for next iteration
        shift = shift
            .checked_shl(7)
            .ok_or_else(|| PatchError::Other("Varint shift overflow".to_string()))?;

        result = result
            .checked_add(shift)
            .ok_or_else(|| PatchError::Other("Varint addition overflow".to_string()))?;

        // Safety check: varints shouldn't be unreasonably long
        if bytes_read > 10 {
            // 10 bytes = 70 bits, more than enough for u64
            return Err(PatchError::InvalidFormat(
                "Varint too long (>10 bytes)".to_string(),
            ));
        }
    }

    Err(PatchError::UnexpectedEof(
        "Incomplete varint at end of data".to_string(),
    ))
}
