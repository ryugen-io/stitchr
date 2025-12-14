//! xdelta format implementation (VCDIFF)
//!
//! xdelta is a binary diff tool that uses the VCDIFF format (RFC 3284).
//! It is commonly used for patching larger files like NDS/PS2/PSP games.

mod address_cache;
mod checksum;
mod code_table;
mod decoder;

use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

const VCDIFF_HEADER: &[u8] = &[0xd6, 0xc3, 0xc4]; // 'V' | 0x80, 'C' | 0x80, 'D' | 0x80

/// xdelta format patcher
pub struct XdeltaPatcher;

impl PatchFormat for XdeltaPatcher {
    fn can_handle(data: &[u8]) -> bool {
        data.len() >= 3 && &data[0..3] == VCDIFF_HEADER
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidMagic {
                expected: VCDIFF_HEADER.to_vec(),
                actual: patch.get(0..3).unwrap_or(&[]).to_vec(),
            });
        }

        decoder::apply_patch(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not an xdelta patch".to_string()));
        }

        Ok(PatchMetadata::new(PatchType::Xdelta))
    }

    fn validate(patch: &[u8]) -> Result<()> {
        if !Self::can_handle(patch) {
            return Err(PatchError::InvalidFormat("Not an xdelta patch".to_string()));
        }
        Ok(())
    }
}
