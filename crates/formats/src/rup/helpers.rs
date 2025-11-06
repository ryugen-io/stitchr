//! RUP helper functions

use super::constants::*;
use rom_patcher_core::{PatchError, Result};

/// Compute MD5 hash of data
pub fn compute_md5(data: &[u8]) -> [u8; 16] {
    let digest = md5::compute(data);
    digest.0
}

/// Validate MD5 hash
pub fn validate_md5(data: &[u8], expected: &[u8; 16]) -> Result<()> {
    let actual = compute_md5(data);
    if &actual != expected {
        return Err(PatchError::Other(format!(
            "MD5 mismatch: expected {}, got {}",
            hex_string(expected),
            hex_string(&actual)
        )));
    }
    Ok(())
}

/// Convert bytes to hex string
fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Parse a null-terminated string from patch data
///
/// Handles literal '\n' sequences by converting them to actual newlines
pub fn parse_metadata_string(patch: &[u8], offset: usize, max_len: usize) -> String {
    if offset + max_len > patch.len() {
        return String::new();
    }

    let slice = &patch[offset..offset + max_len];
    let end = slice.iter().position(|&b| b == 0).unwrap_or(max_len);
    let raw = String::from_utf8_lossy(&slice[..end]).to_string();

    raw.replace("\\n", "\n")
}

/// Parse and validate RUP header
pub fn parse_header(patch: &[u8]) -> Result<()> {
    if patch.len() < HEADER_SIZE {
        return Err(PatchError::InvalidFormat(format!(
            "Patch too small (expected at least {} bytes)",
            HEADER_SIZE
        )));
    }

    if &patch[..MAGIC_SIZE] != MAGIC {
        return Err(PatchError::InvalidMagic {
            expected: MAGIC.to_vec(),
            actual: patch.get(..MAGIC_SIZE).unwrap_or(&[]).to_vec(),
        });
    }

    Ok(())
}
