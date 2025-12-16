//! xdelta format implementation (VCDIFF)
//!
//! xdelta is a binary diff tool that uses the VCDIFF format (RFC 3284).
//! It is commonly used for patching larger files like NDS/PS2/PSP games.

pub mod address_cache;
mod apply;
mod code_table;
pub mod constants;
pub mod headers;
pub mod metadata;
pub mod parser;
pub mod validate;

use stitchr_core::{PatchError, PatchFormat, PatchMetadata, Result};

/// xdelta format patcher
pub struct XdeltaPatcher;

impl PatchFormat for XdeltaPatcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidMagic {
                expected: constants::VCDIFF_HEADER.to_vec(),
                actual: patch.get(0..3).unwrap_or(&[]).to_vec(),
            });
        }
        apply::apply_patch(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract_metadata(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }
}
