//! APS N64 patch application

mod records;

use super::constants::*;
use super::helpers::{parse_header, validate_source_rom};
use rom_patcher_core::{PatchError, Result};

/// Apply APS N64 patch to ROM
pub fn apply(rom: &[u8], patch: &[u8]) -> Result<Vec<u8>> {
    let (header, mut offset) = parse_header(patch)?;

    if header.header_type != HEADER_TYPE_N64 {
        return Err(PatchError::InvalidFormat(
            "Not an APS N64 patch".to_string(),
        ));
    }

    let mut output = vec![0u8; header.output_size as usize];

    let copy_len = rom.len().min(output.len());
    output[..copy_len].copy_from_slice(&rom[..copy_len]);

    while offset < patch.len() {
        let (record_offset, length) = records::parse_record_header(patch, &mut offset)?;

        if length == RECORD_RLE {
            records::process_rle(patch, &mut offset, record_offset, &mut output)?;
        } else {
            records::process_simple(patch, &mut offset, record_offset, length, &mut output)?;
        }
    }

    Ok(output)
}

/// Verify source ROM matches N64 header requirements
pub fn verify(rom: &[u8], patch: &[u8]) -> Result<()> {
    let (header, _) = parse_header(patch)?;

    if let Some(n64_header) = header.n64_header
        && !validate_source_rom(rom, &n64_header)
    {
        return Err(PatchError::ChecksumMismatch {
            expected: 0,
            actual: 0,
        });
    }

    Ok(())
}
