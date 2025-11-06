//! RUP (Rupture Patches) format support

mod apply;
mod constants;
mod helpers;
pub mod metadata;
pub mod validate;

pub mod varint;

use rom_patcher_core::{PatchFormat, PatchMetadata, Result};

pub struct RupPatcher;

impl PatchFormat for RupPatcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        apply::apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }

    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
        validate::verify(rom, patch, target)
    }
}
