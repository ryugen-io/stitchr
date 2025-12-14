//! BDF patch application

use crate::bdf::validate;
use byteorder::{LittleEndian, ReadBytesExt};
use bzip2::read::BzDecoder;
use rom_patcher_core::Result;
use std::io::{Cursor, Read, Seek, SeekFrom};

/// Apply BDF patch to ROM
pub fn apply_patch(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    validate::validate(patch)?;

    let mut cursor = Cursor::new(patch);
    cursor.seek(SeekFrom::Start(8))?; // Skip magic

    let control_size = cursor.read_u64::<LittleEndian>()? as usize;
    let diff_size = cursor.read_u64::<LittleEndian>()? as usize;
    let patched_size = cursor.read_u64::<LittleEndian>()? as usize;

    if patched_size > crate::bdf::constants::MAX_PATCHED_SIZE {
        return Err(rom_patcher_core::PatchError::InvalidFormat(format!(
            "Target size too large: {} bytes (max {})",
            patched_size,
            crate::bdf::constants::MAX_PATCHED_SIZE
        )));
    }

    // Ensure ROM is large enough for patched_size
    if rom.len() < patched_size {
        rom.resize(patched_size, 0);
    } else if rom.len() > patched_size {
        rom.truncate(patched_size);
    }

    let control_compressed_data =
        &patch[cursor.position() as usize..(cursor.position() as usize + control_size)];
    cursor.seek(SeekFrom::Current(control_size as i64))?;

    let diff_compressed_data =
        &patch[cursor.position() as usize..(cursor.position() as usize + diff_size)];
    cursor.seek(SeekFrom::Current(diff_size as i64))?;

    let extra_compressed_data = &patch[cursor.position() as usize..];

    let mut control_decoder = BzDecoder::new(control_compressed_data);
    let mut diff_decoder = BzDecoder::new(diff_compressed_data);
    let mut extra_decoder = BzDecoder::new(extra_compressed_data);

    let mut control_decompressed = Vec::new();
    control_decoder.read_to_end(&mut control_decompressed)?;
    let mut control_cursor = Cursor::new(control_decompressed);

    let mut diff_decompressed = Vec::new();
    diff_decoder.read_to_end(&mut diff_decompressed)?;
    let mut diff_cursor = Cursor::new(diff_decompressed);

    let mut extra_decompressed = Vec::new();
    extra_decoder.read_to_end(&mut extra_decompressed)?;
    let mut extra_cursor = Cursor::new(extra_decompressed);

    // We need a temporary buffer for the new ROM because we read from old ROM
    // randomly (skip) and write to new ROM sequentially.
    let old_rom = rom.clone();
    let mut new_rom = Vec::with_capacity(patched_size);

    let mut old_pos: i64 = 0;

    while control_cursor.position() < control_cursor.get_ref().len() as u64 {
        let diff_len = control_cursor.read_u64::<LittleEndian>()? as usize;
        let extra_len = control_cursor.read_u64::<LittleEndian>()? as usize;
        let skip_len = control_cursor.read_i64::<LittleEndian>()?;

        // Read diff data
        for _ in 0..diff_len {
            let old_byte = if old_pos >= 0 && (old_pos as usize) < old_rom.len() {
                old_rom[old_pos as usize]
            } else {
                0
            };
            old_pos = old_pos.wrapping_add(1);

            let diff_byte = diff_cursor.read_u8()?;
            new_rom.push(old_byte.wrapping_add(diff_byte));
        }

        // Read extra data
        for _ in 0..extra_len {
            let extra_byte = extra_cursor.read_u8()?;
            new_rom.push(extra_byte);
        }

        // Adjust position in old_rom
        old_pos = old_pos.wrapping_add(skip_len);
    }

    *rom = new_rom;
    Ok(())
}
