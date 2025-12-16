//! PPF (PlayStation Patch Format) metadata extraction.
//!
//! This module provides functionality to extract metadata from PPF patch files.

use crate::ppf::helpers::parse_header;
use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

/// Extracts metadata from a PPF patch.
///
/// # Arguments
///
/// * `patch` - The patch data.
///
/// # Returns
///
/// * `Result<PatchMetadata>` - The extracted metadata, or an error if
///   extraction fails.
pub fn extract_metadata(patch: &[u8]) -> Result<PatchMetadata> {
    let mut cursor = Cursor::new(patch);
    let header = parse_header(&mut cursor)?;

    let mut metadata = PatchMetadata::new(PatchType::Ppf);
    metadata = metadata.with_extra("version".to_string(), format!("PPF{}", header.version));

    if !header.description.is_empty() {
        metadata = metadata.with_extra("description".to_string(), header.description);
    }

    if header.version == 3 {
        metadata = metadata
            .with_extra(
                "encoding_method".to_string(),
                header.encoding_method.to_string(),
            )
            .with_extra("image_type".to_string(), header.image_type.to_string())
            .with_extra("block_check".to_string(), header.block_check.to_string())
            .with_extra("undo_data".to_string(), header.undo_data.to_string());
    } else if header.version == 2 {
        metadata = metadata.with_extra(
            "input_file_size".to_string(),
            header.input_file_size.to_string(),
        );
        metadata = metadata.with_extra("block_check".to_string(), "true".to_string());
    }

    // Skip Block Check binary if present
    if header.block_check {
        cursor
            .seek(SeekFrom::Current(1024))
            .map_err(|_| PatchError::CorruptedData)?;
    }

    // Scan for FILE_ID.DIZ (@BEG)
    // We scan while parsing records because @BEG is intermixed with the record
    // stream structure (though usually at the end).
    // To do this efficiently without fully parsing every record (which requires
    // logic matching apply), we can use a quick scan or just partial parsing.
    // Given the JS implementation checks for @BEG at the start of every record
    // loop, we should replicate that logic to safely find it.

    const FILE_ID_DIZ_MAGIC: &[u8] = b"@BEG";
    const END_FILE_ID_DIZ_MAGIC: &str = "@END_FILE_ID.DIZ";

    while cursor.position() < patch.len() as u64 {
        let current_pos = cursor.position();

        // Check for @BEG
        if (patch.len() as u64 - current_pos) >= 4 {
            let mut magic_check = [0u8; 4];
            if cursor.read_exact(&mut magic_check).is_ok() && magic_check == FILE_ID_DIZ_MAGIC {
                // Found FILE_ID.DIZ
                // JS: readString(3072), substring to @END_FILE_ID.DIZ
                let remaining = patch.len() as u64 - cursor.position();
                let read_len = std::cmp::min(remaining, 3072) as usize;
                let mut buffer = vec![0u8; read_len];
                if cursor.read_exact(&mut buffer).is_ok() {
                    let text = String::from_utf8_lossy(&buffer);
                    if let Some(end_idx) = text.find(END_FILE_ID_DIZ_MAGIC) {
                        let diz_content = text[..end_idx].trim().to_string();
                        if !diz_content.is_empty() {
                            metadata = metadata.with_extra("file_id_diz".to_string(), diz_content);
                        }
                    }
                }
                break;
            }
            cursor.set_position(current_pos);
        } else {
            break;
        }

        // Skip record to advance
        // Offset
        if header.version == 3 {
            cursor
                .seek(SeekFrom::Current(8))
                .map_err(|_| PatchError::CorruptedData)?;
        } else {
            cursor
                .seek(SeekFrom::Current(4))
                .map_err(|_| PatchError::CorruptedData)?;
        }

        // Length
        let mut len_buf = [0u8; 1];
        cursor
            .read_exact(&mut len_buf)
            .map_err(|_| PatchError::CorruptedData)?;
        let len = len_buf[0] as i64;

        // Data
        cursor
            .seek(SeekFrom::Current(len))
            .map_err(|_| PatchError::CorruptedData)?;

        // Undo Data
        if header.undo_data {
            cursor
                .seek(SeekFrom::Current(len))
                .map_err(|_| PatchError::CorruptedData)?;
        }
    }

    Ok(metadata)
}
