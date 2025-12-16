//! BPS metadata extraction

use super::constants::MAGIC_SIZE;
use super::varint;
use stitchr_core::{PatchError, PatchMetadata, PatchType, Result};

/// Extract metadata from a BPS patch
///
/// Returns patch metadata including source/target sizes and optional metadata
/// string.
pub fn extract_metadata(patch: &[u8]) -> Result<PatchMetadata> {
    if patch.len() < MAGIC_SIZE {
        return Err(PatchError::InvalidFormat("Patch too small".to_string()));
    }

    let mut offset = MAGIC_SIZE;
    let mut metadata = PatchMetadata::new(PatchType::Bps);

    // Parse source size
    let (source_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid source size varint".to_string()))?;
    offset += bytes_read;
    metadata.source_size = Some(source_size as usize);

    // Parse target size
    let (target_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid target size varint".to_string()))?;
    offset += bytes_read;
    metadata.target_size = Some(target_size as usize);

    // Parse metadata size and content
    let (metadata_size, bytes_read) = varint::decode(&patch[offset..])
        .map_err(|_| PatchError::InvalidFormat("Invalid metadata size varint".to_string()))?;
    offset += bytes_read;

    if metadata_size > 0 {
        let metadata_end = offset
            .checked_add(metadata_size as usize)
            .ok_or_else(|| PatchError::InvalidFormat("Metadata size too large".to_string()))?;

        if metadata_end > patch.len() {
            return Err(PatchError::UnexpectedEof(
                "Metadata extends beyond patch".to_string(),
            ));
        }

        // Try to parse as UTF-8 string (recommended format is XML UTF-8)
        if let Ok(metadata_str) = String::from_utf8(patch[offset..metadata_end].to_vec()) {
            metadata = metadata.with_extra("metadata".to_string(), metadata_str);
        }
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic_metadata() {
        // BPS1 + source_size=0 + target_size=0 + metadata_size=0
        let patch = b"BPS1\x80\x80\x80";
        let meta = extract_metadata(patch).unwrap();

        assert_eq!(meta.patch_type, PatchType::Bps);
        assert_eq!(meta.source_size, Some(0));
        assert_eq!(meta.target_size, Some(0));
        assert!(meta.extra.is_empty());
    }

    #[test]
    fn test_extract_with_metadata() {
        // BPS1 + source_size=0 + target_size=0 + metadata_size=5 + "hello"
        let mut patch = vec![];
        patch.extend_from_slice(b"BPS1");
        patch.push(0x80); // 0 encoded
        patch.push(0x80); // 0 encoded
        patch.push(0x85); // 5 encoded
        patch.extend_from_slice(b"hello");

        let meta = extract_metadata(&patch).unwrap();

        assert_eq!(meta.source_size, Some(0));
        assert_eq!(meta.target_size, Some(0));
        assert_eq!(meta.extra.len(), 1);
        assert_eq!(meta.extra[0].0, "metadata");
        assert_eq!(meta.extra[0].1, "hello");
    }
}
