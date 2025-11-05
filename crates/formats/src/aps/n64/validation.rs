//! APS N64 ROM validation

use super::constants::*;
use super::types::N64Header;

/// Validate source ROM against N64 header
pub fn validate_source_rom(rom: &[u8], n64_header: &N64Header) -> bool {
    // Check minimum ROM size
    if rom.len() < N64_CRC_OFFSET + N64_CRC_LEN {
        return false;
    }

    // Validate cart ID
    if rom[N64_CART_ID_OFFSET..N64_CART_ID_OFFSET + N64_CART_ID_LEN] != n64_header.cart_id {
        return false;
    }

    // Validate CRC
    if rom[N64_CRC_OFFSET..N64_CRC_OFFSET + N64_CRC_LEN] != n64_header.crc {
        return false;
    }

    true
}
