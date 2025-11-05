//! APS N64 record parsing and processing

use rom_patcher_core::{PatchError, Result};

/// Process a single RLE record
pub fn process_rle(
    patch: &[u8],
    offset: &mut usize,
    record_offset: usize,
    output: &mut [u8],
) -> Result<()> {
    if *offset + 2 > patch.len() {
        return Err(PatchError::UnexpectedEof(
            "Incomplete RLE record".to_string(),
        ));
    }

    let byte_value = patch[*offset];
    *offset += 1;
    let count = patch[*offset] as usize;
    *offset += 1;

    if record_offset + count > output.len() {
        return Err(PatchError::OutOfBounds {
            offset: record_offset,
            rom_size: output.len(),
        });
    }

    for i in 0..count {
        output[record_offset + i] = byte_value;
    }

    Ok(())
}

/// Process a simple data record
pub fn process_simple(
    patch: &[u8],
    offset: &mut usize,
    record_offset: usize,
    length: u8,
    output: &mut [u8],
) -> Result<()> {
    let data_len = length as usize;
    if *offset + data_len > patch.len() {
        return Err(PatchError::UnexpectedEof(
            "Incomplete simple record".to_string(),
        ));
    }

    if record_offset + data_len > output.len() {
        return Err(PatchError::OutOfBounds {
            offset: record_offset,
            rom_size: output.len(),
        });
    }

    output[record_offset..record_offset + data_len]
        .copy_from_slice(&patch[*offset..*offset + data_len]);
    *offset += data_len;

    Ok(())
}

/// Parse record header and return (offset, length)
pub fn parse_record_header(patch: &[u8], offset: &mut usize) -> Result<(usize, u8)> {
    if *offset + 5 > patch.len() {
        return Err(PatchError::UnexpectedEof(
            "Incomplete record header".to_string(),
        ));
    }

    let record_offset = u32::from_le_bytes([
        patch[*offset],
        patch[*offset + 1],
        patch[*offset + 2],
        patch[*offset + 3],
    ]) as usize;
    *offset += 4;

    let length = patch[*offset];
    *offset += 1;

    Ok((record_offset, length))
}
