//! APS N64 validation functions

use super::constants::*;
use super::helpers::parse_header;
use super::record_validation;
use stitchr_core::{PatchError, Result};

/// Check if data is a valid APS N64 patch
pub fn can_handle(data: &[u8]) -> bool {
    if data.len() < MAGIC_LEN {
        return false;
    }

    if &data[..MAGIC_LEN] != MAGIC {
        return false;
    }

    parse_header(data).is_ok()
}

/// Validate patch structure
pub fn validate(patch: &[u8]) -> Result<()> {
    if patch.len() < MIN_PATCH_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch file too small".to_string(),
        ));
    }

    if &patch[..MAGIC_LEN] != MAGIC {
        return Err(PatchError::InvalidMagic {
            expected: MAGIC.to_vec(),
            actual: patch.get(..MAGIC_LEN).unwrap_or(&[]).to_vec(),
        });
    }

    let (header, offset) = parse_header(patch)?;

    if header.header_type != HEADER_TYPE_N64 {
        return Err(PatchError::InvalidFormat(
            "Not an APS N64 patch".to_string(),
        ));
    }

    record_validation::validate_records(patch, offset)?;

    Ok(())
}
