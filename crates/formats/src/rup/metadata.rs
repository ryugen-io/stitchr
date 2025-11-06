//! RUP metadata extraction

use super::constants::*;
use super::helpers;
use super::varint::decode_vlv;
use rom_patcher_core::{PatchError, PatchMetadata, PatchType, Result};

/// RUP patch metadata (header fields)
#[derive(Debug, Clone, PartialEq)]
pub struct RupMetadata {
    pub text_encoding: u8,
    pub author: String,
    pub version: String,
    pub title: String,
    pub genre: String,
    pub language: String,
    pub date: String,
    pub web: String,
    pub description: String,
}

impl RupMetadata {
    /// Extract metadata from RUP patch header
    pub fn from_patch(patch: &[u8]) -> Self {
        Self {
            text_encoding: patch.get(OFFSET_TEXT_ENCODING).copied().unwrap_or(0),
            author: helpers::parse_metadata_string(patch, OFFSET_AUTHOR, SIZE_AUTHOR),
            version: helpers::parse_metadata_string(patch, OFFSET_VERSION, SIZE_VERSION),
            title: helpers::parse_metadata_string(patch, OFFSET_TITLE, SIZE_TITLE),
            genre: helpers::parse_metadata_string(patch, OFFSET_GENRE, SIZE_GENRE),
            language: helpers::parse_metadata_string(patch, OFFSET_LANGUAGE, SIZE_LANGUAGE),
            date: helpers::parse_metadata_string(patch, OFFSET_DATE, SIZE_DATE),
            web: helpers::parse_metadata_string(patch, OFFSET_WEB, SIZE_WEB),
            description: helpers::parse_metadata_string(
                patch,
                OFFSET_DESCRIPTION,
                SIZE_DESCRIPTION,
            ),
        }
    }
}

/// File-specific metadata (parsed during apply)
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub file_name: String,
    pub rom_type: u8,
    pub source_size: u64,
    pub target_size: u64,
    pub source_md5: [u8; 16],
    pub target_md5: [u8; 16],
    pub overflow_mode: Option<u8>,
    pub overflow_data: Vec<u8>,
}

impl FileMetadata {
    /// Get ROM type name
    pub fn rom_type_name(&self) -> &str {
        ROM_TYPE_NAMES
            .get(self.rom_type as usize)
            .copied()
            .unwrap_or("unknown")
    }
}

/// Extract metadata from RUP patch
pub fn extract(patch: &[u8]) -> Result<PatchMetadata> {
    helpers::parse_header(patch)?;

    let rup_meta = RupMetadata::from_patch(patch);
    let mut meta = PatchMetadata::new(PatchType::Rup);

    // Extract first file metadata
    let mut offset = HEADER_SIZE;
    while offset < patch.len() {
        let command = patch[offset];
        offset += 1;

        if command == COMMAND_OPEN_NEW_FILE {
            let (name_len, consumed) = decode_vlv(&patch[offset..])?;
            offset += consumed + name_len as usize + 1;

            let (source_size, consumed) = decode_vlv(&patch[offset..])?;
            offset += consumed;
            let (target_size, _consumed) = decode_vlv(&patch[offset..])?;

            meta.source_size = Some(source_size as usize);
            meta.target_size = Some(target_size as usize);
            break;
        } else if command == COMMAND_END {
            return Err(PatchError::InvalidFormat("No files in patch".to_string()));
        }
    }

    if !rup_meta.author.is_empty() {
        meta = meta.with_extra("author".to_string(), rup_meta.author);
    }
    if !rup_meta.title.is_empty() {
        meta = meta.with_extra("title".to_string(), rup_meta.title);
    }
    if !rup_meta.version.is_empty() {
        meta = meta.with_extra("version".to_string(), rup_meta.version);
    }

    Ok(meta)
}
