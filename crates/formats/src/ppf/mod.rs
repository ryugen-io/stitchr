//! PPF (PlayStation Patch Format) format implementation
//!
//! PPF is primarily used for PlayStation 1 and 2 game patches.
//! Versions: PPF1, PPF2, PPF3

pub mod apply;
pub mod constants;
pub mod helpers;
pub mod metadata;
pub mod validate;

use stitchr_core::{PatchFormat, PatchMetadata, Result};

/// PPF format patcher
pub struct PpfPatcher;

impl PatchFormat for PpfPatcher {
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
        validate::validate_patch(patch)
    }
}
