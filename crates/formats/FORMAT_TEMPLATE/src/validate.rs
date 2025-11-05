//! FORMAT_NAME validation

use super::constants::{MAGIC, MAGIC_SIZE};
use super::helpers;
use rom_patcher_core::{PatchError, Result};

/// Check if data is a valid FORMAT_NAME patch
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= MAGIC_SIZE && &data[0..MAGIC_SIZE] == MAGIC
}

/// Validate FORMAT_NAME patch format
pub fn validate(patch: &[u8]) -> Result<()> {
    // Check magic
    if !can_handle(patch) {
        return Err(PatchError::InvalidFormat(
            "Invalid FORMAT_NAME magic".to_string(),
        ));
    }

    // Check minimum size
    // Validate checksums (if format supports it)
    // helpers::validate_patch_crc(patch)?;

    Ok(())
}

/// Verify ROM checksums (if format supports it)
pub fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
    // Implement if format has ROM checksums
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle() {
        assert!(can_handle(b"MAGIC_HERE\x00\x00"));
        assert!(!can_handle(b"WRONG"));
    }
}
