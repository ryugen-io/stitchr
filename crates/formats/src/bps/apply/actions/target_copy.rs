//! TARGET_COPY action handler

use super::ActionContext;
use crate::bps::helpers::decode_signed_delta;
use crate::bps::varint;
use stitchr_core::{PatchError, Result};

/// Execute TARGET_COPY action (RLE-style)
/// Can have overlapping copies - target grows as we copy!
#[inline]
pub fn target_copy(ctx: &mut ActionContext, length: usize) -> Result<()> {
    ctx.check_growth(length)?;
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
