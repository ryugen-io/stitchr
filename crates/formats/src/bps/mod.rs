//! BPS (Beat Patching System) format implementation
//!
//! BPS is a modern patching format created by byuu/Near that improves upon IPS:
//! - Supports files of any size (not limited to 16MB)
//! - CRC32 checksums for source, target, and patch validation
//! - Variable-length integer encoding for efficiency
//! - Four action types: SourceRead, TargetRead, SourceCopy, TargetCopy
//!
//! Format specification: Public domain by byuu

use rom_patcher_core::{PatchFormat, PatchMetadata, Result};

mod apply;
mod helpers;
mod metadata;
mod validate;
mod varint;

/// BPS format constants
pub mod constants {
    /// BPS magic header ("BPS1")
    pub const MAGIC: &[u8] = b"BPS1";
    pub const MAGIC_SIZE: usize = 4;

    /// Footer size: 3x CRC32 checksums
    pub const FOOTER_SIZE: usize = 12;

    /// Action types (encoded in command byte low 2 bits)
    pub const ACTION_SOURCE_READ: u8 = 0;
    pub const ACTION_TARGET_READ: u8 = 1;
    pub const ACTION_SOURCE_COPY: u8 = 2;
    pub const ACTION_TARGET_COPY: u8 = 3;
}

/// BPS format patcher
pub struct BpsPatcher;

impl PatchFormat for BpsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        apply::apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        metadata::extract_metadata(patch)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }

    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
        // Verify source ROM checksum
        helpers::validate_source_crc(rom, patch)?;

        // Verify target ROM checksum if provided
        if let Some(target_data) = target {
            helpers::validate_target_crc(target_data, patch)?;
        }

        Ok(())
    }
}
