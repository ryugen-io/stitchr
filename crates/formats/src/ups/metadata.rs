//! UPS metadata extraction

use super::constants::*;
use super::helpers;
use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};

/// Extract metadata from UPS patch
pub fn extract(patch: &[u8]) -> Result<PatchMetadata> {
    // Validate magic
    if patch.len() < MAGIC_SIZE || &patch[..MAGIC_SIZE] != MAGIC {
        return Err(PatchError::InvalidFormat("Invalid UPS magic".to_string()));
    }

    // Parse header
    let (input_size, output_size, _offset) = helpers::parse_header(patch)?;

    // Extract checksums
    let input_crc = u32::from_le_bytes([
        patch[patch.len() - 12],
        patch[patch.len() - 11],
        patch[patch.len() - 10],
        patch[patch.len() - 9],
    ]);
    let output_crc = u32::from_le_bytes([
        patch[patch.len() - 8],
        patch[patch.len() - 7],
        patch[patch.len() - 6],
        patch[patch.len() - 5],
    ]);

    Ok(PatchMetadata {
        patch_type: PatchType::Ups,
        source_size: Some(input_size as usize),
        target_size: Some(output_size as usize),
        source_checksum: Some(input_crc.to_le_bytes().to_vec()),
        target_checksum: Some(output_crc.to_le_bytes().to_vec()),
        extra: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic_metadata() {
        let mut patch = Vec::new();
        patch.extend_from_slice(b"UPS1");
        patch.push(0x8A); // Input size = 10
        patch.push(0x94); // Output size = 20

        let input_rom = vec![0u8; 10];
        let input_crc = crc32fast::hash(&input_rom);
        patch.extend_from_slice(&input_crc.to_le_bytes());

        let output_rom = vec![0u8; 20];
        let output_crc = crc32fast::hash(&output_rom);
        patch.extend_from_slice(&output_crc.to_le_bytes());

        let patch_crc = crc32fast::hash(&patch);
        patch.extend_from_slice(&patch_crc.to_le_bytes());

        let metadata = extract(&patch).unwrap();
        assert_eq!(metadata.source_size, Some(10));
        assert_eq!(metadata.target_size, Some(20));
    }
}
