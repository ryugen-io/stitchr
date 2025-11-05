//! IPS (International Patching System) format support

use rom_patcher_core::{PatchFormat, PatchMetadata, Result};

mod apply;
mod constants;
mod io;
mod metadata;
mod validate;

pub use constants::{MAX_RECORD_SIZE, MAX_ROM_SIZE};

/// IPS format patcher
pub struct IpsPatcher;

impl PatchFormat for IpsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        apply::can_handle(data)
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
}
