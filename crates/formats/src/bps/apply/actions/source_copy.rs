//! SOURCE_COPY action handler

use super::ActionContext;
use crate::bps::helpers::decode_signed_delta;
use crate::bps::varint;
use stitchr_core::{PatchError, Result};

/// Execute SOURCE_COPY action
#[inline]
pub fn source_copy(ctx: &mut ActionContext, length: usize) -> Result<()> {
    ctx.check_growth(length)?;
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
