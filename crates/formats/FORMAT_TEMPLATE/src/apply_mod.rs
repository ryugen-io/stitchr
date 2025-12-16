//! FORMAT_NAME patch application
//!
//! This file should be placed at: src/FORMAT_NAME/apply/mod.rs

use super::constants::*;
use super::helpers::*;
use stitchr_core::{PatchError, Result};

/// Apply a FORMAT_NAME patch to a ROM
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    // Validate magic
    if !super::validate::can_handle(patch) {
        return Err(PatchError::InvalidFormat(
            "Invalid FORMAT_NAME magic".to_string(),
        ));
    }

    // Parse header
    let (_input_size, output_size, mut offset) = parse_header(patch)?;

    // Resize ROM if needed
    rom.resize(output_size as usize, 0);

    // Apply patch records/actions
    // ...

    Ok(())
}
