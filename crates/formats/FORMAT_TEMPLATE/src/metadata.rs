//! FORMAT_NAME metadata extraction

use super::constants::*;
use super::helpers;
use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};

/// Extract metadata from FORMAT_NAME patch
pub fn extract(patch: &[u8]) -> Result<PatchMetadata> {
    if !super::validate::can_handle(patch) {
        return Err(PatchError::InvalidFormat(
            "Invalid FORMAT_NAME magic".to_string(),
        ));
    }

    // Parse header
    let (input_size, output_size, _offset) = helpers::parse_header(patch)?;

    Ok(PatchMetadata {
        patch_type: PatchType::Unknown, // Change to actual type
        source_size: Some(input_size as usize),
        target_size: Some(output_size as usize),
        source_checksum: None, // Extract if format supports it
        target_checksum: None, // Extract if format supports it
        extra: Vec::new(),
    })
}
