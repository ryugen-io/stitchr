//! IPS patch application

mod records;

use crate::ips::constants::{EOF_MARKER, HEADER};
use crate::ips::io::read_u24_be;
use rom_patcher_core::{PatchError, Result};

/// Apply IPS patch to ROM data
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    validate_header(patch)?;

    let mut offset = HEADER.len();
    let patch_len = patch.len();

    while offset + 3 <= patch_len {
        let record_offset = read_u24_be(&patch[offset..offset + 3]);
        offset += 3;

        if record_offset == EOF_MARKER {
            handle_eof(rom, patch, offset)?;
            return Ok(());
        }

        offset = records::apply_record(rom, patch, offset, record_offset as usize)?;
    }

    Err(PatchError::InvalidFormat("Missing EOF marker".to_string()))
}

/// Validate IPS header
fn validate_header(patch: &[u8]) -> Result<()> {
    if patch.len() < HEADER.len() || &patch[0..HEADER.len()] != HEADER {
        return Err(PatchError::InvalidMagic {
            expected: HEADER.to_vec(),
            actual: patch.get(0..HEADER.len()).unwrap_or(&[]).to_vec(),
        });
    }
    Ok(())
}

/// Handle EOF marker and optional truncation
fn handle_eof(rom: &mut Vec<u8>, patch: &[u8], offset: usize) -> Result<()> {
    if offset + 3 <= patch.len() {
        let truncate_size = read_u24_be(&patch[offset..offset + 3]);
        rom.truncate(truncate_size as usize);
    }
    Ok(())
}

/// Check if data is a valid IPS patch (magic header check)
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= HEADER.len() && &data[0..HEADER.len()] == HEADER
}
