//! xdelta metadata extraction

use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};

use super::validate;

/// Extract metadata from xdelta patch
pub fn extract_metadata(patch: &[u8]) -> Result<PatchMetadata> {
    if !validate::can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not an xdelta patch".to_string()));
    }

    Ok(PatchMetadata::new(PatchType::Xdelta))
}
