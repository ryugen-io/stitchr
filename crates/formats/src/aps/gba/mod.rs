pub mod apply;
pub mod constants;
pub mod helpers;
pub mod metadata;
pub mod validate;

use rom_patcher_core::{PatchFormat, PatchMetadata, PatchType, Result};

pub struct ApsGbaPatcher;

impl PatchFormat for ApsGbaPatcher {
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
        result.source_size = Some(meta.source_size as usize);
        result.target_size = Some(meta.target_size as usize);
        result
            .extra
            .push(("Source Size".to_string(), meta.source_size.to_string()));
        result
            .extra
            .push(("Target Size".to_string(), meta.target_size.to_string()));
        result
            .extra
            .push(("Record Count".to_string(), meta.record_count.to_string()));
        Ok(result)
    }
}
