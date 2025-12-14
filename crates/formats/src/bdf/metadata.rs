//! BDF metadata extraction

use crate::bdf::constants::{BDF_MAGIC, HEADER_SIZE};
use byteorder::{LittleEndian, ReadBytesExt};
use rom_patcher_core::{PatchError, PatchMetadata, PatchType, Result};
use std::io::{Cursor, Seek, SeekFrom};

/// Extract metadata from BDF patch
pub fn extract_metadata(patch: &[u8]) -> Result<PatchMetadata> {
    if patch.len() < 8 || &patch[0..8] != BDF_MAGIC {
        return Err(PatchError::InvalidFormat("Not a BDF patch".to_string()));
    }

    if patch.len() < HEADER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Truncated BDF header".to_string(),
        ));
    }

    let mut cursor = Cursor::new(patch);
    cursor.seek(SeekFrom::Start(8))?; // Skip magic

    let control_size = cursor.read_u64::<LittleEndian>()?;
    let diff_size = cursor.read_u64::<LittleEndian>()?;
    let patched_size = cursor.read_u64::<LittleEndian>()?;

    let mut metadata = PatchMetadata::new(PatchType::Bdf);
    metadata = metadata
        .with_extra("control_size".to_string(), control_size.to_string())
        .with_extra("diff_size".to_string(), diff_size.to_string())
        .with_extra("patched_size".to_string(), patched_size.to_string());

    Ok(metadata)
}
