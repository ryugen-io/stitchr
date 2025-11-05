//! APS N64 type definitions

use super::constants::*;

/// N64-specific header data
#[derive(Debug, Clone)]
pub struct N64Header {
    pub original_format: u8,
    pub cart_id: [u8; N64_CART_ID_LEN],
    pub crc: [u8; N64_CRC_LEN],
    pub pad: [u8; N64_PAD_LEN],
}

/// APS patch header
#[derive(Debug, Clone)]
pub struct ApsHeader {
    pub header_type: u8,
    pub encoding_method: u8,
    pub description: String,
    pub n64_header: Option<N64Header>,
    pub output_size: u32,
}
