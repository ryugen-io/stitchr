use super::constants::*;
use super::helpers::parse_header;
use rom_patcher_core::Result;

pub fn can_handle(data: &[u8]) -> bool {
    data.len() >= MAGIC_LEN && &data[0..MAGIC_LEN] == MAGIC
}

pub fn validate(patch: &[u8]) -> Result<()> {
    parse_header(patch)?;

    if patch.len() < MIN_PATCH_SIZE {
        return Err(rom_patcher_core::PatchError::InvalidFormat(
            "Patch too small to contain records".to_string(),
        ));
    }

    let remaining = patch.len() - HEADER_SIZE;
    if !remaining.is_multiple_of(RECORD_SIZE) {
        return Err(rom_patcher_core::PatchError::InvalidFormat(
            "Invalid record alignment".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_valid_patch() -> Vec<u8> {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.extend_from_slice(&1024u32.to_le_bytes());
        patch.extend_from_slice(&1024u32.to_le_bytes());
        patch.extend_from_slice(&0u32.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);
        patch
    }

    #[test]
    fn test_can_handle_valid() {
        let patch = make_valid_patch();
        assert!(can_handle(&patch));
    }

    #[test]
    fn test_can_handle_invalid_magic() {
        let patch = b"NOTAPS";
        assert!(!can_handle(patch));
    }

    #[test]
    fn test_validate_minimal() {
        let patch = make_valid_patch();
        assert!(validate(&patch).is_ok());
    }

    #[test]
    fn test_validate_too_small() {
        let patch = b"APS1";
        assert!(validate(patch).is_err());
    }
}
