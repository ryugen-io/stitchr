//! FORMAT_NAME helper functions

use super::constants::*;
use stitchr_core::{PatchError, Result};
use stitchr_features::validation::algorithms::crc32;

/// Parse FORMAT_NAME header
pub fn parse_header(patch: &[u8]) -> Result<(u64, u64, usize)> {
    // Return (input_size, output_size, data_offset)
    todo!("Implement header parsing")
}

/// Validate patch CRC32 (if format has checksums)
pub fn validate_patch_crc(patch: &[u8]) -> Result<()> {
    todo!("Implement CRC validation if format supports it")
}

/// Validate input ROM CRC32 (if format has checksums)
pub fn validate_input_crc(rom: &[u8], patch: &[u8]) -> Result<()> {
    todo!("Implement if format supports it")
}

/// Validate output ROM CRC32 (if format has checksums)
pub fn validate_output_crc(target: &[u8], patch: &[u8]) -> Result<()> {
    todo!("Implement if format supports it")
}
