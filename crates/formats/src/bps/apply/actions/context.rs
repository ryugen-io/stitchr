//! Action execution context

/// Context for action execution
pub struct ActionContext<'a> {
    pub rom: &'a [u8],
    pub patch: &'a [u8],
    pub target: &'a mut Vec<u8>,
    pub source_relative_offset: &'a mut i64,
    pub target_relative_offset: &'a mut i64,
    pub offset: &'a mut usize,
}
