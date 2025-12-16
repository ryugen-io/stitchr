//! FORMAT_NAME (Full Name) format support

mod apply;
mod constants;
mod helpers;
mod metadata;
mod validate;
// mod varint;  // If needed

use stitchr_core::{PatchFormat, PatchMetadata, Result};

pub struct FormatNamePatcher;

impl PatchFormat for FormatNamePatcher {
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
