use super::constants::*;
use stitchr_core::{PatchError, Result};

#[derive(Debug)]
pub struct GbaHeader {
    pub source_size: u32,
    pub target_size: u32,
}

#[derive(Debug)]
pub struct GbaRecord {
    pub offset: u32,
    pub source_crc16: u16,
    pub target_crc16: u16,
    pub xor_data: Vec<u8>,
}

pub fn parse_header(patch: &[u8]) -> Result<(GbaHeader, usize)> {
    if patch.len() < HEADER_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch file too small".to_string(),
        ));
    }

    if &patch[0..MAGIC_LEN] != MAGIC {
        return Err(PatchError::InvalidMagic {
            expected: MAGIC.to_vec(),
            actual: patch[0..MAGIC_LEN.min(patch.len())].to_vec(),
        });
    }

    let source_size = u32::from_le_bytes([patch[4], patch[5], patch[6], patch[7]]);
    let target_size = u32::from_le_bytes([patch[8], patch[9], patch[10], patch[11]]);

    Ok((
        GbaHeader {
            source_size,
            target_size,
        },
        HEADER_SIZE,
    ))
}

pub fn parse_record(patch: &[u8], offset: usize) -> Result<GbaRecord> {
    if offset + RECORD_SIZE > patch.len() {
        return Err(PatchError::UnexpectedEof("Incomplete record".to_string()));
    }

    let record_offset = u32::from_le_bytes([
        patch[offset],
        patch[offset + 1],
        patch[offset + 2],
        patch[offset + 3],
    ]);

    let source_crc16 = u16::from_le_bytes([patch[offset + 4], patch[offset + 5]]);

    let target_crc16 = u16::from_le_bytes([patch[offset + 6], patch[offset + 7]]);

    let xor_data = patch[offset + 8..offset + 8 + BLOCK_SIZE].to_vec();

    Ok(GbaRecord {
        offset: record_offset,
        source_crc16,
        target_crc16,
        xor_data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_header() {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.extend_from_slice(&1024u32.to_le_bytes());
        patch.extend_from_slice(&2048u32.to_le_bytes());

        let (header, offset) = parse_header(&patch).unwrap();
        assert_eq!(header.source_size, 1024);
        assert_eq!(header.target_size, 2048);
        assert_eq!(offset, HEADER_SIZE);
    }
}
