//! IPS metadata extraction

use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};

use super::constants::{EOF_MARKER, HEADER};
use super::io::{read_u16_be, read_u24_be};

/// Extract metadata from IPS patch
///
/// Note: IPS format doesn't store checksums, so we estimate sizes
/// by analyzing patch records.
pub fn extract(patch: &[u8]) -> Result<PatchMetadata> {
    validate_patch(patch)?;

    let mut metadata = PatchMetadata::new(PatchType::Ips);
    let max_offset = calculate_max_offset(patch, &mut metadata)?;

    if metadata.target_size.is_none() {
        metadata.target_size = Some(max_offset);
    }

    Ok(metadata)
}

/// Validate patch format
fn validate_patch(patch: &[u8]) -> Result<()> {
    if patch.len() < HEADER.len() || &patch[0..HEADER.len()] != HEADER {
        return Err(PatchError::InvalidFormat("Not an IPS patch".to_string()));
    }
    Ok(())
}

/// Calculate maximum offset from patch records
fn calculate_max_offset(patch: &[u8], metadata: &mut PatchMetadata) -> Result<usize> {
    let mut max_offset = 0usize;
    let mut offset = HEADER.len();

    while offset + 3 <= patch.len() {
        let record_offset = read_u24_be(&patch[offset..offset + 3]);

        if record_offset == EOF_MARKER {
            if let Some(truncate_size) = try_read_truncate_size(patch, offset + 3) {
                metadata.target_size = Some(truncate_size);
            }
            break;
        }

        offset += 3;
        if offset + 2 > patch.len() {
            break;
        }

        let (new_offset, record_end) = process_record(patch, offset, record_offset as usize)?;
        max_offset = max_offset.max(record_end);
        offset = new_offset;
    }

    Ok(max_offset)
}

/// Try to read truncation size from EOF marker
fn try_read_truncate_size(patch: &[u8], offset: usize) -> Option<usize> {
    if offset + 3 <= patch.len() {
        Some(read_u24_be(&patch[offset..offset + 3]) as usize)
    } else {
        None
    }
}

/// Process a single record and return new offset and record end position
fn process_record(patch: &[u8], mut offset: usize, record_offset: usize) -> Result<(usize, usize)> {
    let size = read_u16_be(&patch[offset..offset + 2]);
    offset += 2;

    let record_end = if size == 0 {
        // RLE record
        if offset + 3 > patch.len() {
            return Err(PatchError::CorruptedData);
        }
        let rle_size = read_u16_be(&patch[offset..offset + 2]) as usize;
        offset += 3;
        record_offset + rle_size
    } else {
        // Normal record
        let size = size as usize;
        if offset + size > patch.len() {
            return Err(PatchError::CorruptedData);
        }
        offset += size;
        record_offset + size
    };

    Ok((offset, record_end))
}
