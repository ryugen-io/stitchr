//! PPF (PlayStation Patch Format) validation.
//!
//! This module provides functionality to validate PPF patch files.

use crate::ppf::{constants::*, helpers::parse_header};
use stitchr_core::{PatchError, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

/// Checks if the provided data can be handled as a PPF patch.
///
/// # Arguments
///
/// * `data` - The byte slice to check.
///
/// # Returns
///
/// * `bool` - True if the data appears to be a PPF patch, false otherwise.
pub fn can_handle(data: &[u8]) -> bool {
    if data.len() < 5 {
        return false;
    }

    &data[0..5] == PPF1_HEADER || &data[0..5] == PPF2_HEADER || &data[0..5] == PPF3_HEADER
}

/// Validates a PPF patch file.
///
/// # Arguments
///
/// * `patch` - The patch data.
///
/// # Returns
///
/// * `Result<()>` - Ok if the patch is valid, an error otherwise.
pub fn validate_patch(patch: &[u8]) -> Result<()> {
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat("Not a PPF patch".to_string()));
    }

    let mut cursor = Cursor::new(patch);
    let header = parse_header(&mut cursor)?;

    // Skip Block Check binary if present
    if header.block_check {
        cursor
            .seek(SeekFrom::Current(1024))
            .map_err(|_| PatchError::CorruptedData)?;
    }

    const FILE_ID_DIZ_MAGIC: &[u8] = b"@BEG";

    while cursor.position() < patch.len() as u64 {
        let current_pos = cursor.position();

        // Check for @BEG (FILE_ID.DIZ)
        if (patch.len() as u64 - current_pos) >= 4 {
            let mut magic_check = [0u8; 4];
            if cursor.read_exact(&mut magic_check).is_ok() && magic_check == FILE_ID_DIZ_MAGIC {
                // Found metadata, validation successful up to here
                // We could validate the DIZ structure but matching magic is sufficient for
                // structural integrity
                return Ok(());
            }
            cursor.set_position(current_pos);
        } else if cursor.position() == patch.len() as u64 {
            break; // EOF
        }

        // Validate Offset
        if header.version == 3 {
            cursor
                .seek(SeekFrom::Current(8))
                .map_err(|_| PatchError::CorruptedData)?;
        } else {
            cursor
                .seek(SeekFrom::Current(4))
                .map_err(|_| PatchError::CorruptedData)?;
        }

        // Validate Length
        let mut len_buf = [0u8; 1];
        cursor
            .read_exact(&mut len_buf)
            .map_err(|_| PatchError::CorruptedData)?;
        let len = len_buf[0] as i64;

        // Validate Data
        cursor
            .seek(SeekFrom::Current(len))
            .map_err(|_| PatchError::CorruptedData)?;

        // Validate Undo Data
        if header.undo_data {
            cursor
                .seek(SeekFrom::Current(len))
                .map_err(|_| PatchError::CorruptedData)?;
        }
    }

    // Ensure we consumed exactly the expected amount or hit EOF cleanly
    if cursor.position() > patch.len() as u64 {
        return Err(PatchError::CorruptedData);
    }

    Ok(())
}
