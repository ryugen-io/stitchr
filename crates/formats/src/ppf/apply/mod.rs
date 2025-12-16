//! PPF (PlayStation Patch Format) patching logic.
//!
//! This module contains the core logic for applying PPF patches.

use crate::ppf::{constants::*, validate::*};
use byteorder::{LittleEndian, ReadBytesExt};
use stitchr_core::{PatchError, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

/// Applies a PPF patch to the provided ROM data.
///
/// # Arguments
///
/// * `rom` - A mutable reference to the ROM data.
/// * `patch` - The patch data.
///
/// # Returns
///
/// * `Result<()>` - Ok if the patch was applied successfully, an error
///   otherwise.
pub fn apply_patch(rom: &mut [u8], patch: &[u8]) -> Result<()> {
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
    }

    let mut patch_cursor = Cursor::new(patch);

    // 1. Read Magic (5 bytes)
    let mut header_bytes = [0u8; 5];
    patch_cursor
        .read_exact(&mut header_bytes)
        .map_err(|_| PatchError::CorruptedData)?;

    let version = if header_bytes == PPF1_HEADER {
        1
    } else if header_bytes == PPF2_HEADER {
        2
    } else if header_bytes == PPF3_HEADER {
        3
    } else {
        return Err(PatchError::InvalidFormat("Unknown PPF version".to_string()));
    };

    // 2. Read Encoding Method (1 byte)
    let _encoding_method = patch_cursor
        .read_u8()
        .map_err(|_| PatchError::CorruptedData)?;

    // 3. Read Description (50 bytes) - skip it
    patch_cursor
        .seek(SeekFrom::Current(50))
        .map_err(|_| PatchError::CorruptedData)?;

    let mut block_check = false;
    let mut undo_data = false;

    // 4. Version specific headers
    match version {
        1 => {
            // PPF1 has no extra header fields
        }
        2 => {
            // PPF2 has Input File Size (u32) and implicit Block Check
            let _input_file_size = patch_cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| PatchError::CorruptedData)?;
            block_check = true;
        }
        3 => {
            // PPF3: ImageType(1), BlockCheck(1), UndoData(1), Dummy(1)
            let _image_type = patch_cursor
                .read_u8()
                .map_err(|_| PatchError::CorruptedData)?;
            let block_check_byte = patch_cursor
                .read_u8()
                .map_err(|_| PatchError::CorruptedData)?;
            block_check = block_check_byte != 0;
            let undo_data_byte = patch_cursor
                .read_u8()
                .map_err(|_| PatchError::CorruptedData)?;
            undo_data = undo_data_byte != 0;
            // Skip dummy byte
            patch_cursor
                .seek(SeekFrom::Current(1))
                .map_err(|_| PatchError::CorruptedData)?;
        }
        _ => {
            return Err(PatchError::InvalidFormat(
                "Unsupported PPF version".to_string(),
            ));
        }
    }

    // 5. Block Check (Validation Binary)
    if block_check {
        patch_cursor
            .seek(SeekFrom::Current(1024))
            .map_err(|_| PatchError::CorruptedData)?;
    }

    // 6. Records Loop
    const FILE_ID_DIZ_MAGIC: &[u8] = b"@BEG";

    while patch_cursor.position() < patch.len() as u64 {
        // Check for @BEG (FILE_ID.DIZ)
        let current_pos = patch_cursor.position();
        if (patch.len() as u64 - current_pos) >= 4 {
            let mut magic_check = [0u8; 4];
            if patch_cursor.read_exact(&mut magic_check).is_ok() && magic_check == FILE_ID_DIZ_MAGIC
            {
                break; // Reached metadata section
            }
            // Rewind if not @BEG
            patch_cursor.set_position(current_pos);
        } else if patch_cursor.position() == patch.len() as u64 {
            break; // EOF
        }

        // Read Offset
        let offset = if version == 3 {
            patch_cursor
                .read_u64::<LittleEndian>()
                .map_err(|_| PatchError::CorruptedData)?
        } else {
            patch_cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| PatchError::CorruptedData)? as u64
        };

        // Read Length (u8)
        let data_len = patch_cursor
            .read_u8()
            .map_err(|_| PatchError::CorruptedData)? as usize;

        // Read Data
        let mut data = vec![0u8; data_len];
        patch_cursor
            .read_exact(&mut data)
            .map_err(|_| PatchError::CorruptedData)?;

        // Apply to ROM
        let end_offset = (offset as usize)
            .checked_add(data_len)
            .ok_or_else(|| PatchError::Other("Offset + data_len overflow".to_string()))?;

        if end_offset > rom.len() {
            return Err(PatchError::OutOfBounds {
                offset: offset as usize,
                rom_size: rom.len(),
            });
        }

        rom[offset as usize..end_offset].copy_from_slice(&data);

        // Skip Undo Data if present
        if undo_data {
            patch_cursor
                .seek(SeekFrom::Current(data_len as i64))
                .map_err(|_| PatchError::CorruptedData)?;
        }
    }

    Ok(())
}
