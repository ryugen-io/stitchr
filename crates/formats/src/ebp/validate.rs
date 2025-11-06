//! EBP validation

use super::constants::*;
use rom_patcher_core::{PatchError, Result};

/// Validate EBP patch format
pub fn validate(patch: &[u8]) -> Result<()> {
    // Check minimum size (magic + EOF)
    if patch.len() < MAGIC_SIZE + EOF_MARKER.len() {
        return Err(PatchError::InvalidFormat("EBP patch too small".to_string()));
    }

    // Check magic (same as IPS)
    if &patch[..MAGIC_SIZE] != MAGIC {
        return Err(PatchError::InvalidFormat(format!(
            "Invalid EBP magic: expected {:?}, got {:?}",
            MAGIC,
            &patch[..MAGIC_SIZE]
        )));
    }

    // Check for EOF marker
    if !patch.windows(EOF_MARKER.len()).any(|w| w == EOF_MARKER) {
        return Err(PatchError::InvalidFormat("Missing EOF marker".to_string()));
    }

    Ok(())
}

/// Check if patch can be handled
pub fn can_handle(patch: &[u8]) -> bool {
    patch.len() >= MAGIC_SIZE && &patch[..MAGIC_SIZE] == MAGIC
}
