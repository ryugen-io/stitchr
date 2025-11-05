//! Variable-length integer encoding/decoding for BPS format
//!
//! BPS uses a custom varint encoding that supports arbitrary file sizes.
//! Each byte stores 7 bits of data, with the 8th bit as a continuation flag.

use rom_patcher_core::{PatchError, Result};

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
pub fn decode(data: &[u8]) -> Result<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift: u64 = 1;
    let mut bytes_read = 0;

    for &byte in data.iter() {
        bytes_read += 1;

        // Add lower 7 bits multiplied by shift
        result = result
            .checked_add((byte & 0x7f) as u64 * shift)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_single_byte() {
        // 0x80 = 0b10000000 -> value 0, termination bit set
        assert_eq!(decode(&[0x80]).unwrap(), (0, 1));

        // 0x81 = 0b10000001 -> value 1
        assert_eq!(decode(&[0x81]).unwrap(), (1, 1));

        // 0xFF = 0b11111111 -> value 127
        assert_eq!(decode(&[0xFF]).unwrap(), (127, 1));
    }

    #[test]
    fn test_decode_multi_byte() {
        // 0x00 0x80 -> 128
        assert_eq!(decode(&[0x00, 0x80]).unwrap(), (128, 2));

        // 0x01 0x80 -> 129
        assert_eq!(decode(&[0x01, 0x80]).unwrap(), (129, 2));
    }

    #[test]
    fn test_decode_overflow() {
        // Too many bytes without termination
        let long_varint = vec![0x00; 11];
        assert!(decode(&long_varint).is_err());
    }

    #[test]
    fn test_decode_incomplete() {
        // No termination bit
        assert!(decode(&[0x00]).is_err());
        assert!(decode(&[0x7F]).is_err());
    }
}
