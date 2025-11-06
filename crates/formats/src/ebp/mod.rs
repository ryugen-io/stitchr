//! EBP format implementation
//!
//! EBP is an extension of IPS that adds optional JSON metadata.
//! The patch data itself is IPS-compatible.

mod constants;
mod helpers;
pub mod metadata;
pub mod validate;

pub mod apply;

use rom_patcher_core::{PatchError, PatchFormat, PatchMetadata, PatchType, Result};

/// EBP patch format handler
pub struct EbpPatcher;

impl PatchFormat for EbpPatcher {
    fn can_handle(data: &[u8]) -> bool {
        validate::can_handle(data)
    }

    fn validate(patch: &[u8]) -> Result<()> {
        validate::validate(patch)
    }

    fn apply(&self, rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
        Self::validate(patch)?;
        apply::apply(rom, patch)
    }

    fn metadata(patch: &[u8]) -> Result<PatchMetadata> {
        let ebp_meta = metadata::EbpMetadata::from_patch(patch);
        let mut meta = PatchMetadata::new(PatchType::Ebp);

        if let Some(title) = ebp_meta.title {
            meta = meta.with_extra("title".to_string(), title);
        }
        if let Some(author) = ebp_meta.author {
            meta = meta.with_extra("author".to_string(), author);
        }
        if let Some(description) = ebp_meta.description {
            meta = meta.with_extra("description".to_string(), description);
        }
        if let Some(version) = ebp_meta.version {
            meta = meta.with_extra("version".to_string(), version);
        }

        Ok(meta)
    }

    fn verify(_rom: &[u8], _patch: &[u8], _target: Option<&[u8]>) -> Result<()> {
        // EBP has no checksums (same as IPS)
        Err(PatchError::Other(
            "EBP format does not support checksum verification".to_string(),
        ))
    }
}
