//! APS (Advance Patching System) format support

pub mod gba;
pub mod n64;

use stitchr_core::{PatchFormat, PatchMetadata, Result};

/// APS format patcher
pub struct ApsPatcher;

impl PatchFormat for ApsPatcher {
    fn can_handle(data: &[u8]) -> bool {
        n64::ApsN64Patcher::can_handle(data) || gba::ApsGbaPatcher::can_handle(data)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        if n64::ApsN64Patcher::can_handle(patch) {
            n64::ApsN64Patcher.apply(rom, patch)
        } else if gba::ApsGbaPatcher::can_handle(patch) {
            gba::ApsGbaPatcher.apply(rom, patch)
        } else {
            Err(stitchr_core::PatchError::InvalidFormat(
                "Unknown APS variant".to_string(),
            ))
        }
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        if n64::ApsN64Patcher::can_handle(patch) {
            n64::ApsN64Patcher::metadata(patch)
        } else if gba::ApsGbaPatcher::can_handle(patch) {
            gba::ApsGbaPatcher::metadata(patch)
        } else {
            Err(stitchr_core::PatchError::InvalidFormat(
                "Unknown APS variant".to_string(),
            ))
        }
    }

    fn validate(patch: &[u8]) -> Result<()> {
        if n64::ApsN64Patcher::can_handle(patch) {
            n64::ApsN64Patcher::validate(patch)
        } else if gba::ApsGbaPatcher::can_handle(patch) {
            gba::ApsGbaPatcher::validate(patch)
        } else {
            Err(stitchr_core::PatchError::InvalidFormat(
                "Unknown APS variant".to_string(),
            ))
        }
    }

    fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
        if n64::ApsN64Patcher::can_handle(patch) {
            n64::ApsN64Patcher::verify(rom, patch, target)
        } else if gba::ApsGbaPatcher::can_handle(patch) {
            gba::ApsGbaPatcher::verify(rom, patch, target)
        } else {
            Err(stitchr_core::PatchError::InvalidFormat(
                "Unknown APS variant".to_string(),
            ))
        }
    }
}
