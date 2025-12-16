//! UPS format validation

use super::constants::{FOOTER_SIZE, MAGIC, MAGIC_SIZE};
use super::helpers;
use stitchr_core::{PatchError, Result};

/// Check if data is a valid UPS patch (magic header check)
pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= MAGIC_SIZE && &data[0..MAGIC_SIZE] == MAGIC
}

/// Validate UPS patch format and patch CRC32
pub fn validate(patch: &[u8]) -> Result<()> {
    // Check magic
    if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
        return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
    }

    // Check minimum size
    if patch.len() < MAGIC_SIZE + FOOTER_SIZE {
        return Err(PatchError::InvalidFormat("Patch too small".to_string()));
    }

    // Validate patch CRC32
    helpers::validate_patch_crc(patch)?;

    Ok(())
}

/// Verify ROM checksums (input or output)
pub fn verify(rom: &[u8], patch: &[u8], target: Option<&[u8]>) -> Result<()> {
    if let Some(target_rom) = target {
        // Verify output ROM
        helpers::validate_output_crc(target_rom, patch)?;
    } else {
        // Verify input ROM
        helpers::validate_input_crc(rom, patch)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle() {
        assert!(can_handle(
            b"UPS1\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
        ));
        assert!(!can_handle(b"IPS\x00"));
        assert!(!can_handle(b"UPS"));
        assert!(!can_handle(b""));
    }

    #[test]
    fn test_validate_rejects_too_small() {
        let invalid_patch = b"UPS1";
        assert!(validate(invalid_patch).is_err());
    }

    #[test]
    fn test_validate_rejects_invalid_magic() {
        let invalid_patch = b"NOPE";
        assert!(validate(invalid_patch).is_err());
    }
}
