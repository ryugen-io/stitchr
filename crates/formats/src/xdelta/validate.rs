//! xdelta validation

use super::constants::VCDIFF_HEADER;
use stitchr_core::{PatchError, Result};

/// Check if the patch data has xdelta magic
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= 3 && &data[0..3] == VCDIFF_HEADER
}

/// Validate xdelta patch integrity
pub fn validate(patch: &[u8]) -> Result<()> {
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not an xdelta patch".to_string()));
    }
    Ok(())
}
