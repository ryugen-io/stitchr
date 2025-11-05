use super::constants::*;
use super::helpers::parse_header;
use rom_patcher_core::Result;

pub struct Metadata {
    pub source_size: u32,
    pub target_size: u32,
    pub record_count: usize,
}

pub fn extract_metadata(patch: &[u8]) -> Result<Metadata> {
    let (header, _) = parse_header(patch)?;

    let remaining = patch.len().saturating_sub(HEADER_SIZE);
    let record_count = remaining / RECORD_SIZE;

    Ok(Metadata {
        source_size: header.source_size,
        target_size: header.target_size,
        record_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_metadata() {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.extend_from_slice(&1024u32.to_le_bytes());
        patch.extend_from_slice(&2048u32.to_le_bytes());
        patch.extend_from_slice(&0u32.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        patch.extend_from_slice(&vec![0u8; BLOCK_SIZE]);

        let meta = extract_metadata(&patch).unwrap();
        assert_eq!(meta.source_size, 1024);
        assert_eq!(meta.target_size, 2048);
        assert_eq!(meta.record_count, 1);
    }
}
