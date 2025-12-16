//! SOURCE_READ action handler

use super::ActionContext;
use stitchr_core::{PatchError, Result};

/// Execute SOURCE_READ action
/// Copies bytes from ROM at current output position (not relative offset!)
#[inline]
pub fn source_read(ctx: &mut ActionContext, length: usize) -> Result<()> {
    ctx.check_growth(length)?;
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
