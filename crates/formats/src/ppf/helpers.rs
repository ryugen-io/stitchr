//! PPF (PlayStation Patch Format) helpers.
//!
//! This module provides helper functions for parsing PPF headers and
//! structures.

use crate::ppf::constants::*;
use byteorder::{LittleEndian, ReadBytesExt};
use stitchr_core::{PatchError, Result};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct PpfHeader {
    pub version: u8,
    pub description: String,
    pub encoding_method: u8,
    pub image_type: u8,
    pub block_check: bool,
    pub undo_data: bool,
    pub input_file_size: u32, // For PPF2
}

pub fn parse_header<R: Read + Seek>(cursor: &mut R) -> Result<PpfHeader> {
    // 1. Read Magic (5 bytes)
    let mut header_bytes = [0u8; 5];
    cursor
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
    let encoding_method = cursor.read_u8().map_err(|_| PatchError::CorruptedData)?;

    // 3. Read Description (50 bytes)
    let mut desc_bytes = [0u8; 50];
    cursor
        .read_exact(&mut desc_bytes)
        .map_err(|_| PatchError::CorruptedData)?;

    // Trim nulls and whitespace from description
    let description = String::from_utf8_lossy(&desc_bytes)
        .trim_matches(char::from(0))
        .trim()
        .to_string();

    let mut header = PpfHeader {
        version,
        description,
        encoding_method,
        image_type: 0,
        block_check: false,
        undo_data: false,
        input_file_size: 0,
    };

    // 4. Version specific headers
    match version {
        1 => {
            // PPF1 has no extra header fields
        }
        2 => {
            // PPF2 has Input File Size (u32) and implicit Block Check
            header.input_file_size = cursor
                .read_u32::<LittleEndian>()
                .map_err(|_| PatchError::CorruptedData)?;
            header.block_check = true;
        }
        3 => {
            // PPF3: ImageType(1), BlockCheck(1), UndoData(1), Dummy(1)
            header.image_type = cursor.read_u8().map_err(|_| PatchError::CorruptedData)?;
            let block_check_byte = cursor.read_u8().map_err(|_| PatchError::CorruptedData)?;
            header.block_check = block_check_byte != 0;
            let undo_data_byte = cursor.read_u8().map_err(|_| PatchError::CorruptedData)?;
            header.undo_data = undo_data_byte != 0;
            // Skip dummy byte
            cursor
                .seek(SeekFrom::Current(1))
                .map_err(|_| PatchError::CorruptedData)?;
        }
        _ => {
            return Err(PatchError::InvalidFormat(
                "Unsupported PPF version".to_string(),
            ));
        }
    }

    Ok(header)
}
