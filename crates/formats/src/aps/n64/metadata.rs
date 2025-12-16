//! APS N64 metadata extraction

use super::helpers::parse_header;
use stitchr_core::Result;

/// Metadata extracted from APS N64 patch
#[derive(Debug, Clone)]
pub struct Metadata {
    pub output_size: u32,
    pub description: String,
    pub cart_id: Option<String>,
    pub crc: Option<String>,
}

/// Extract metadata from APS N64 patch
pub fn extract_metadata(patch: &[u8]) -> Result<Metadata> {
    let (header, _) = parse_header(patch)?;

    let (cart_id, crc) = if let Some(n64_header) = header.n64_header {
        // Convert cart ID to string
        let cart_id_str = String::from_utf8_lossy(&n64_header.cart_id).to_string();

        // Convert CRC to hex string
        let crc_str = n64_header
            .crc
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        (Some(cart_id_str), Some(crc_str))
    } else {
        (None, None)
    };

    Ok(Metadata {
        output_size: header.output_size,
        description: header.description,
        cart_id,
        crc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aps::n64::constants::*;

    fn create_test_patch() -> Vec<u8> {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.push(HEADER_TYPE_N64);
        patch.push(0x00);

        // Description
        let mut desc = b"Test Patch".to_vec();
        desc.resize(DESCRIPTION_LEN, 0);
        patch.extend_from_slice(&desc);

        // N64 header
        patch.push(0x01);
        patch.extend_from_slice(b"NTE"); // Cart ID
        patch.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]); // CRC
        patch.extend_from_slice(&[0u8; N64_PAD_LEN]);

        // Output size
        patch.extend_from_slice(&2048u32.to_le_bytes());

        patch
    }

    #[test]
    fn test_extract_metadata() {
        let patch = create_test_patch();
        let metadata = extract_metadata(&patch).unwrap();

        assert_eq!(metadata.output_size, 2048);
        assert_eq!(metadata.description, "Test Patch");
        assert_eq!(metadata.cart_id, Some("NTE".to_string()));
        assert_eq!(metadata.crc, Some("0102030405060708".to_string()));
    }
}
