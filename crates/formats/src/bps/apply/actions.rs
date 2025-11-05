//! BPS action handlers

use crate::bps::helpers::decode_signed_delta;
use crate::bps::varint;
use rom_patcher_core::{PatchError, Result};

/// Context for action execution
pub struct ActionContext<'a> {
    pub rom: &'a [u8],
    pub patch: &'a [u8],
    pub target: &'a mut Vec<u8>,
    pub source_relative_offset: &'a mut i64,
    pub target_relative_offset: &'a mut i64,
    pub offset: &'a mut usize,
}

/// Execute SOURCE_READ action
/// Copies bytes from ROM at current output position (not relative offset!)
#[inline]
pub fn source_read(ctx: &mut ActionContext, length: usize) -> Result<()> {
    let source_offset = ctx.target.len(); // Use current output position, not source_relative_offset!

    if source_offset + length > ctx.rom.len() {
        return Err(PatchError::InvalidFormat(
            "SourceRead exceeds source bounds".to_string(),
        ));
    }

    ctx.target
        .extend_from_slice(&ctx.rom[source_offset..source_offset + length]);
    Ok(())
}

/// Execute TARGET_READ action
#[inline]
pub fn target_read(ctx: &mut ActionContext, length: usize) -> Result<()> {
    if *ctx.offset + length > ctx.patch.len() {
        return Err(PatchError::UnexpectedEof(
            "TargetRead exceeds patch bounds".to_string(),
        ));
    }

    ctx.target
        .extend_from_slice(&ctx.patch[*ctx.offset..*ctx.offset + length]);
    *ctx.offset += length;
    Ok(())
}

/// Execute SOURCE_COPY action
#[inline]
pub fn source_copy(ctx: &mut ActionContext, length: usize) -> Result<()> {
    let (data, bytes_read) = varint::decode(&ctx.patch[*ctx.offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid SourceCopy offset".to_string()))?;
    *ctx.offset += bytes_read;

    *ctx.source_relative_offset += decode_signed_delta(data);

    if *ctx.source_relative_offset < 0
        || *ctx.source_relative_offset as usize + length > ctx.rom.len()
    {
        return Err(PatchError::InvalidFormat(
            "SourceCopy offset out of bounds".to_string(),
        ));
    }

    let source_offset = *ctx.source_relative_offset as usize;
    ctx.target
        .extend_from_slice(&ctx.rom[source_offset..source_offset + length]);
    *ctx.source_relative_offset += length as i64;
    Ok(())
}

/// Execute TARGET_COPY action (RLE-style)
/// Can have overlapping copies - target grows as we copy!
#[inline]
pub fn target_copy(ctx: &mut ActionContext, length: usize) -> Result<()> {
    let (data, bytes_read) = varint::decode(&ctx.patch[*ctx.offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid TargetCopy offset".to_string()))?;
    *ctx.offset += bytes_read;

    *ctx.target_relative_offset += decode_signed_delta(data);

    // Only check that START position is valid - target grows as we copy (RLE)
    if *ctx.target_relative_offset < 0 || *ctx.target_relative_offset as usize >= ctx.target.len() {
        return Err(PatchError::InvalidFormat(
            "TargetCopy offset out of bounds".to_string(),
        ));
    }

    let target_offset = *ctx.target_relative_offset as usize;

    // Reserve capacity upfront to avoid reallocations
    ctx.target.reserve(length);

    // Handle overlapping copies (RLE-style) - target grows as we copy
    // Use extend_from_within for efficient bulk copying
    let pattern_size = ctx.target.len() - target_offset;

    if pattern_size >= length {
        // No overlap: simple extend_from_within
        ctx.target
            .extend_from_within(target_offset..target_offset + length);
    } else {
        // Overlapping copy: double the pattern each iteration for O(log n) performance
        let mut copied = 0;
        while copied < length {
            let available = ctx.target.len() - target_offset;
            let chunk_size = available.min(length - copied);
            ctx.target
                .extend_from_within(target_offset..target_offset + chunk_size);
            copied += chunk_size;
        }
    }

    *ctx.target_relative_offset += length as i64;
    Ok(())
}
