//! BPS patch application

mod actions;

use super::constants::*;
use super::helpers::*;
use super::varint;
use actions::ActionContext;
use rom_patcher_core::{PatchError, Result};

/// Apply a BPS patch to a ROM
pub fn apply(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    if patch.len() < MAGIC_SIZE + FOOTER_SIZE {
        return Err(PatchError::InvalidFormat("BPS patch too small".to_string()));
    }

    let (source_size, target_size, mut offset) = parse_header(patch)?;

    if rom.len() != source_size as usize {
        return Err(PatchError::SizeMismatch {
            expected: source_size as usize,
            actual: rom.len(),
        });
    }

    validate_source_crc(rom, patch)?;

    let mut target = Vec::with_capacity(target_size as usize);
    let mut source_relative_offset: i64 = 0;
    let mut target_relative_offset: i64 = 0;
    let commands_end = patch.len() - FOOTER_SIZE;

    while offset < commands_end {
        let (command, bytes_read) = varint::decode(&patch[offset..])
            .map_err(|_| PatchError::InvalidFormat("Invalid command varint".to_string()))?;
        offset += bytes_read;

        let action = (command & 0x03) as u8;
        let length = ((command >> 2) + 1) as usize;

        let mut ctx = ActionContext {
            rom,
            patch,
            target: &mut target,
            source_relative_offset: &mut source_relative_offset,
            target_relative_offset: &mut target_relative_offset,
            offset: &mut offset,
        };

        match action {
            ACTION_SOURCE_READ => actions::source_read(&mut ctx, length)?,
            ACTION_TARGET_READ => actions::target_read(&mut ctx, length)?,
            ACTION_SOURCE_COPY => actions::source_copy(&mut ctx, length)?,
            ACTION_TARGET_COPY => actions::target_copy(&mut ctx, length)?,
            _ => {
                return Err(PatchError::InvalidFormat(format!(
                    "Unknown action type: {}",
                    action
                )));
            }
        }
    }

    if target.len() != target_size as usize {
        return Err(PatchError::SizeMismatch {
            expected: target_size as usize,
            actual: target.len(),
        });
    }
    validate_target_crc(&target, patch)?;

    *rom = target;
    Ok(())
}
