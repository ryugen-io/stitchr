//! BDF validation

use crate::bdf::constants::{BDF_MAGIC, HEADER_SIZE};
use byteorder::{LittleEndian, ReadBytesExt};
use rom_patcher_core::{PatchError, Result};
use std::io::{Cursor, Seek, SeekFrom};

/// Check if the patch data has BDF magic
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= 8 && &data[0..8] == BDF_MAGIC
}

/// Validate BDF patch integrity
pub fn validate(patch: &[u8]) -> Result<()> {
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not a BDF patch".to_string()));
    }

    if patch.len() < HEADER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Truncated BDF header".to_string(),
        ));
    }

    let mut cursor = Cursor::new(patch);
    cursor.seek(SeekFrom::Start(8))?; // Skip magic

    let control_size = cursor.read_u64::<LittleEndian>()? as usize;
    let diff_size = cursor.read_u64::<LittleEndian>()? as usize;

    let required_len = HEADER_SIZE
        .checked_add(control_size)
        .and_then(|s| s.checked_add(diff_size));

    if let Some(total_len) = required_len {
        if patch.len() < total_len {
            return Err(PatchError::CorruptedData);
        }
    } else {
        // Overflow in size calculation means sizes are too large for usize/patch
        return Err(PatchError::CorruptedData);
    }

    Ok(())
}
