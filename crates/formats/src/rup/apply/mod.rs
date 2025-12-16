//! RUP patch application

mod file;
mod overflow;
mod records;

use super::constants::*;
use super::helpers::*;
use file::{FileMeta, parse_file_metadata};
use records::apply_xor_records;
use stitchr_core::{PatchError, Result};

/// Apply RUP patch to ROM
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    parse_header(patch)?;

    let rom_md5 = compute_md5(rom);
    let (file_meta, undo) = find_matching_file(patch, &rom_md5)?;

    let target_size = if undo {
        file_meta.source_size
    } else {
        file_meta.target_size
    };

    // Limit target size to prevent ASAN crashes
    const MAX_TARGET_SIZE: u64 = 512 * 1024 * 1024;
    if target_size > MAX_TARGET_SIZE {
        return Err(PatchError::InvalidFormat(format!(
            "Target size too large: {} (max {})",
            target_size, MAX_TARGET_SIZE
        )));
    }

    rom.try_reserve(target_size as usize)
        .map_err(|_| PatchError::Other("Failed to allocate memory for target ROM".to_string()))?;
    rom.resize(target_size as usize, 0);

    apply_xor_records(rom, &file_meta.records)?;

    if let Some(mode) = file_meta.overflow_mode {
        apply_overflow_mode(rom, mode, &file_meta, undo)?;
    }

    let expected_md5 = if undo {
        &file_meta.source_md5
    } else {
        &file_meta.target_md5
    };
    validate_md5(rom, expected_md5)?;

    Ok(())
}

/// Find file in patch matching ROM MD5 (source or target)
fn find_matching_file(patch: &[u8], rom_md5: &[u8; 16]) -> Result<(FileMeta, bool)> {
    let mut offset = HEADER_SIZE;

    while offset < patch.len() {
        let command = patch[offset];
        offset += 1;

        if command == COMMAND_OPEN_NEW_FILE {
            let (file_meta, new_offset) = parse_file_metadata(patch, offset)?;
            offset = new_offset;

            if &file_meta.source_md5 == rom_md5 {
                return Ok((file_meta, false)); // Forward
            } else if &file_meta.target_md5 == rom_md5 {
                return Ok((file_meta, true)); // Undo
            }
        } else if command == COMMAND_END {
            break;
        }
    }

    Err(PatchError::Other(
        "No matching file found (MD5 mismatch)".to_string(),
    ))
}

/// Apply overflow based on mode and direction
fn apply_overflow_mode(
    rom: &mut Vec<u8>,
    mode: u8,
    file_meta: &FileMeta,
    undo: bool,
) -> Result<()> {
    match (mode, undo) {
        (OVERFLOW_APPEND, false) => overflow::apply_append(
            rom,
            &file_meta.overflow_data,
            file_meta.source_size as usize,
        ),
        (OVERFLOW_MINIFY, true) => overflow::apply_minify_undo(
            rom,
            &file_meta.overflow_data,
            file_meta.target_size as usize,
        ),
        _ => Ok(()),
    }
}
