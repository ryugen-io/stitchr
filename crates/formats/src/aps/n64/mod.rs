//! APS N64 format support

pub mod apply;
pub mod constants;
pub mod helpers;
pub mod metadata;
pub mod validate;

mod parsing;
mod record_validation;
mod types;
mod validation;

use stitchr_core::{PatchFormat, PatchMetadata, PatchType, Result};

/// APS N64 patcher
pub struct ApsN64Patcher;

impl PatchFormat for ApsN64Patcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        let result = apply::apply(rom, patch)?;
        *rom = result;
        Ok(())
    }

    fn verify(rom: &[u8], patch: &[u8], _target: Option<&[u8]>) -> Result<()> {
        apply::verify(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        let meta = metadata::extract_metadata(patch)?;

        let mut result = PatchMetadata::new(PatchType::Aps);
        result.target_size = Some(meta.output_size as usize);

        result = result.with_extra("Output Size".to_string(), meta.output_size.to_string());

        if !meta.description.is_empty() {
            result = result.with_extra("Description".to_string(), meta.description);
        }

        if let Some(cart_id) = meta.cart_id {
            result = result.with_extra("Cart ID".to_string(), cart_id);
        }

        if let Some(crc) = meta.crc {
            result = result.with_extra("CRC".to_string(), crc);
        }

        Ok(result)
    }
}
