//! Action execution context

use stitchr_core::{PatchError, Result};

/// Context for action execution
pub struct ActionContext<'a> {
    pub rom: &'a [u8],
    pub patch: &'a [u8],
    pub target: &'a mut Vec<u8>,
    pub source_relative_offset: &'a mut i64,
    pub target_relative_offset: &'a mut i64,
    pub offset: &'a mut usize,
    pub expected_target_size: usize,
}

impl<'a> ActionContext<'a> {
    #[inline]
    pub fn check_growth(&self, length: usize) -> Result<()> {
        if self
            .target
            .len()
            .checked_add(length)
            .ok_or_else(|| PatchError::Other("Target size overflow".to_string()))?
            > self.expected_target_size
        {
            return Err(PatchError::InvalidFormat(format!(
                "Target size exceeded detected: {} > expected {}",
                self.target.len() + length,
                self.expected_target_size
            )));
        }
        Ok(())
    }
}
