//! BPS format validation

use super::constants::{FOOTER_SIZE, MAGIC, MAGIC_SIZE};
use super::varint;
use crc32fast;
use rom_patcher_core::{PatchError, Result};

/// Check if data is a valid BPS patch (magic header check)
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= MAGIC_SIZE && &data[0..MAGIC_SIZE] == MAGIC
}

/// Validate BPS patch format and checksums
pub fn validate(patch: &[u8]) -> Result<()> {
    // Check minimum size (header + footer)
    if patch.len() < MAGIC_SIZE + FOOTER_SIZE {
        return Err(PatchError::InvalidFormat(format!(
            "BPS patch too small: {} bytes (minimum {})",
            patch.len(),
            MAGIC_SIZE + FOOTER_SIZE
        )));
    }

    // Check magic
    if !can_handle(patch) {
        return Err(PatchError::InvalidMagic {
            expected: MAGIC.to_vec(),
            actual: patch.get(0..MAGIC_SIZE).unwrap_or(&[]).to_vec(),
        });
    }

    // Parse header to ensure varints are valid
    let mut offset = MAGIC_SIZE;

    // Source size
    let (_, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid source size varint".to_string()))?;
    offset += bytes_read;

    // Target size
    let (_, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid target size varint".to_string()))?;
    offset += bytes_read;

    // Metadata size
    let (metadata_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid metadata size varint".to_string()))?;
    offset += bytes_read;

    // Check metadata bounds
    if offset + metadata_size as usize > patch.len() - FOOTER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Metadata extends beyond patch bounds".to_string(),
        ));
    }

    // Validate patch CRC32 (last 4 bytes of footer)
    let patch_data_end = patch.len() - 4;
    let stored_patch_crc = u32::from_le_bytes([
        patch[patch_data_end],
        patch[patch_data_end + 1],
        patch[patch_data_end + 2],
        patch[patch_data_end + 3],
    ]);

    let computed_patch_crc = crc32fast::hash(&patch[..patch_data_end]);

    if stored_patch_crc != computed_patch_crc {
        return Err(PatchError::ChecksumMismatch {
            expected: stored_patch_crc,
            actual: computed_patch_crc,
        });
    }

    Ok(())
}
