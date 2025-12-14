//! VCDIFF window header decoding

use crate::xdelta::{
    constants::{VCD_ADLER32, VCD_SOURCE, VCD_TARGET},
    parser::VcdiffParser,
};
use rom_patcher_core::{PatchError, Result};
use std::io::Read;

pub struct WindowHeader {
    pub indicator: u8,
    pub source_length: u64,
    pub source_position: u64,
    pub delta_length: u64,
    pub target_window_length: u64,
    pub add_run_data_length: u64,
    pub instructions_length: u64,
    pub addresses_length: u64,
    pub adler32: Option<u32>,
}

impl WindowHeader {
    pub fn decode(parser: &mut VcdiffParser) -> Result<Self> {
        let indicator = parser.read_u8()?;
        let mut source_length = 0;
        let mut source_position = 0;

        if (indicator & (VCD_SOURCE | VCD_TARGET)) != 0 {
            source_length = parser.read_7bit_encoded_int()?;
            source_position = parser.read_7bit_encoded_int()?;
        }

        let delta_length = parser.read_7bit_encoded_int()?;
        let target_window_length = parser.read_7bit_encoded_int()?;
        // Safety limit: 256MB per window to prevent allocation DoS
        if target_window_length > 256 * 1024 * 1024 {
            return Err(PatchError::InvalidFormat(format!(
                "Window size too large: {} bytes (max 256MB)",
                target_window_length
            )));
        }
        let delta_indicator = parser.read_u8()?;

        if delta_indicator != 0 {
            return Err(PatchError::Other(format!(
                "Unimplemented window header delta indicator: {}",
                delta_indicator
            )));
        }

        let add_run_data_length = parser.read_7bit_encoded_int()?;
        let instructions_length = parser.read_7bit_encoded_int()?;
        let addresses_length = parser.read_7bit_encoded_int()?;

        let mut adler32 = None;
        if (indicator & VCD_ADLER32) != 0 {
            let mut buf = [0u8; 4];
            parser
                .cursor
                .read_exact(&mut buf)
                .map_err(|_| PatchError::CorruptedData)?;
            // RFC 3284 says network byte order (Big Endian)
            adler32 = Some(u32::from_be_bytes(buf));
        }

        Ok(WindowHeader {
            indicator,
            source_length,
            source_position,
            delta_length,
            target_window_length,
            add_run_data_length,
            instructions_length,
            addresses_length,
            adler32,
        })
    }
}

pub fn calculate_target_size(patch: &[u8], offset: u64) -> Result<u64> {
    let mut parser = VcdiffParser::new_at_offset(patch, offset);
    let mut target_size = 0;
    while !parser.is_eof() {
        let win_header = WindowHeader::decode(&mut parser)?;
        target_size += win_header.target_window_length;
        parser.skip(
            win_header.add_run_data_length
                + win_header.instructions_length
                + win_header.addresses_length,
        )?;
    }
    Ok(target_size)
}
