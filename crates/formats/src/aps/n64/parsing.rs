//! APS N64 header parsing

use super::constants::*;
use super::types::{ApsHeader, N64Header};
use rom_patcher_core::{PatchError, Result};

/// Parse APS header from patch data
pub fn parse_header(patch: &[u8]) -> Result<(ApsHeader, usize)> {
    if patch.len() < MIN_PATCH_SIZE {
        return Err(PatchError::InvalidFormat(
            "Patch file too small".to_string(),
        ));
    }

    let mut offset = MAGIC_LEN;

    // Read header type and encoding method
    let header_type = patch[offset];
    offset += 1;
    let encoding_method = patch[offset];
    offset += 1;

    // Read description (50 bytes, null-terminated)
    let desc_bytes = &patch[offset..offset + DESCRIPTION_LEN];
    let description = String::from_utf8_lossy(desc_bytes)
        .trim_end_matches('\0')
        .to_string();
    offset += DESCRIPTION_LEN;

    // Parse N64-specific header if type is N64
    let n64_header = if header_type == HEADER_TYPE_N64 {
        if patch.len() < offset + N64_HEADER_SIZE + 4 {
            return Err(PatchError::InvalidFormat(
                "Patch file too small for N64 header".to_string(),
            ));
        }

        let original_format = patch[offset];
        offset += 1;

        let mut cart_id = [0u8; N64_CART_ID_LEN];
        cart_id.copy_from_slice(&patch[offset..offset + N64_CART_ID_LEN]);
        offset += N64_CART_ID_LEN;

        let mut crc = [0u8; N64_CRC_LEN];
        crc.copy_from_slice(&patch[offset..offset + N64_CRC_LEN]);
        offset += N64_CRC_LEN;

        let mut pad = [0u8; N64_PAD_LEN];
        pad.copy_from_slice(&patch[offset..offset + N64_PAD_LEN]);
        offset += N64_PAD_LEN;

        Some(N64Header {
            original_format,
            cart_id,
            crc,
            pad,
        })
    } else {
        None
    };

    // Read output size (little-endian u32)
    let output_size = u32::from_le_bytes([
        patch[offset],
        patch[offset + 1],
        patch[offset + 2],
        patch[offset + 3],
    ]);
    offset += 4;

    Ok((
        ApsHeader {
            header_type,
            encoding_method,
            description,
            n64_header,
            output_size,
        },
        offset,
    ))
}
