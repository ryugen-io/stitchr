//! APS N64 record validation

use super::constants::*;
use rom_patcher_core::{PatchError, Result};

/// Validate all records in patch data
pub fn validate_records(patch: &[u8], mut offset: usize) -> Result<()> {
    while offset < patch.len() {
        if offset + 5 > patch.len() {
            return Err(PatchError::UnexpectedEof(
                "Incomplete record header".to_string(),
            ));
        }

        offset += 4; // Skip record offset
        let length = patch[offset];
        offset += 1;

        if length == RECORD_RLE {
            if offset + 2 > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete RLE record".to_string(),
                ));
            }
            offset += 2;
        } else {
            if offset + length as usize > patch.len() {
                return Err(PatchError::UnexpectedEof(
                    "Incomplete simple record".to_string(),
                ));
            }
            offset += length as usize;
        }
    }

    Ok(())
}
