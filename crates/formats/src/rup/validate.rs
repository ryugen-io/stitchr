//! RUP format validation

use super::constants::{HEADER_SIZE, MAGIC, MAGIC_SIZE};
use super::helpers;
use stitchr_core::{PatchError, Result};

/// Check if data is a valid RUP patch (magic header check)
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= MAGIC_SIZE && &data[0..MAGIC_SIZE] == MAGIC
}

/// Validate RUP patch format
pub fn validate(patch: &[u8]) -> Result<()> {
    // Check magic and minimum size
    helpers::parse_header(patch)?;

    // Ensure patch has data section after header
    if patch.len() <= HEADER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch has no data section".to_string(),
        ));
    }

    Ok(())
}

/// Verify ROM checksums (source or target MD5)
pub fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
    use super::varint;

    let mut offset = HEADER_SIZE;

    // Find first OPEN_NEW_FILE command
    while offset < patch.len() {
        let command = patch[offset];
        offset += 1;

        if command == super::constants::COMMAND_OPEN_NEW_FILE {
            // Parse file metadata to get MD5 hashes
            let (file_name_len, consumed) = varint::decode_vlv(&patch[offset..])?;
            offset += consumed + file_name_len as usize;
            offset += 1; // rom_type

            let (_source_size, consumed) = varint::decode_vlv(&patch[offset..])?;
            offset += consumed;
            let (_target_size, consumed) = varint::decode_vlv(&patch[offset..])?;
            offset += consumed;

            if offset + 32 > patch.len() {
                return Err(PatchError::UnexpectedEof("MD5 hashes".to_string()));
            }

            let mut source_md5 = [0u8; 16];
            let mut target_md5 = [0u8; 16];
            source_md5.copy_from_slice(&patch[offset..offset + 16]);
            target_md5.copy_from_slice(&patch[offset + 16..offset + 32]);

            if let Some(target_rom) = target {
                helpers::validate_md5(target_rom, &target_md5)?;
            } else {
                helpers::validate_md5(rom, &source_md5)?;
            }

            return Ok(());
        } else if command == super::constants::COMMAND_END {
            return Err(PatchError::InvalidFormat("No files in patch".to_string()));
        }
    }

    Err(PatchError::InvalidFormat(
        "No file metadata found".to_string(),
    ))
}
