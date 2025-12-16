//! Validation utilities for CLI

#[cfg(feature = "validation")]
use stitchr_features::validation::{HashAlgorithm, ValidationFeature, Validator};

/// Compute CRC32 checksum of data
#[cfg(feature = "validation")]
pub fn compute_crc32(data: &[u8]) -> u32 {
    let validator = Validator::new();
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/// Format CRC32 as hex string
pub fn format_crc32(crc: u32) -> String {
    format!("{:08x}", crc)
}
