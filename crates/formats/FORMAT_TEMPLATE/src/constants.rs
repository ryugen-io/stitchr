//! FORMAT_NAME constants

/// Magic bytes for FORMAT_NAME patches
pub const MAGIC: &[u8] = b"MAGIC_HERE";
pub const MAGIC_SIZE: usize = MAGIC.len();

/// Footer size (if applicable)
pub const FOOTER_SIZE: usize = 0; // Adjust based on format

// Add format-specific constants here
