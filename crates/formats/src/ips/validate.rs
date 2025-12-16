//! IPS patch validation

use stitchr_core::{PatchError, Result};

use super::constants::{EOF_MARKER, HEADER};
use super::io::{read_u16_be, read_u24_be};

/// Validate IPS patch structure
///
/// Checks for:
/// - Valid magic header
/// - Proper record structure
/// - Presence of EOF marker
pub fn validate(patch: &[u8]) -> Result<()> {
    validate_header(patch)?;
    validate_records(patch)?;
    Ok(())
}

/// Validate header magic bytes
fn validate_header(patch: &[u8]) -> Result<()> {
    if patch.len() < HEADER.len() || &patch[0..HEADER.len()] != HEADER {
        return Err(PatchError::InvalidFormat("Not an IPS patch".to_string()));
    }
    Ok(())
}

/// Validate all records in patch
fn validate_records(patch: &[u8]) -> Result<()> {
    let mut offset = HEADER.len();
    let patch_len = patch.len();

    while offset + 3 <= patch_len {
        let record_offset = read_u24_be(&patch[offset..offset + 3]);
        offset += 3;

        if record_offset == EOF_MARKER {
            return Ok(()); // Valid EOF found
        }

        offset = validate_record(patch, offset, patch_len)?;
    }

    Err(PatchError::InvalidFormat("Missing EOF marker".to_string()))
}

/// Validate a single record
fn validate_record(patch: &[u8], mut offset: usize, patch_len: usize) -> Result<usize> {
    if offset + 2 > patch_len {
        return Err(PatchError::CorruptedData);
    }

    let size = read_u16_be(&patch[offset..offset + 2]);
    offset += 2;

    if size == 0 {
        // RLE record: needs 2 bytes (size) + 1 byte (value)
        if offset + 3 > patch_len {
            return Err(PatchError::CorruptedData);
        }
        Ok(offset + 3)
    } else {
        // Normal record: needs `size` bytes of data
        if offset + size as usize > patch_len {
            return Err(PatchError::CorruptedData);
        }
        Ok(offset + size as usize)
    }
}
