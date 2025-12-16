//! BDF (Binary Diff Format) implementation
//!
//! Based on bsdiff format (BSDIFF40 magic), which uses bzip2 compression.

use stitchr_core::{PatchFormat, PatchMetadata, Result};

pub mod apply;
pub mod constants;
pub mod metadata;
pub mod validate;

/// BDF format patcher
pub struct BdfPatcher;

impl PatchFormat for BdfPatcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        apply::apply_patch(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract_metadata(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }
}
