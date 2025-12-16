//! VCDIFF decoder implementation

use crate::xdelta::{
    address_cache::AddressCache,
    code_table::{VCD_ADD, VCD_COPY, VCD_NOOP, VCD_RUN, get_default_code_table},
};
use stitchr_features::validation::algorithms::adler32;
use stitchr_core::{PatchError, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

// Header Indicator
const VCD_DECOMPRESS: u8 = 0x01;
const VCD_CODETABLE: u8 = 0x02;
const VCD_APPHEADER: u8 = 0x04;

// Window Indicator
const VCD_SOURCE: u8 = 0x01;
const VCD_TARGET: u8 = 0x02;
const VCD_ADLER32: u8 = 0x04;

// Address Modes
const VCD_MODE_SELF: u8 = 0;
const VCD_MODE_HERE: u8 = 1;

pub struct VcdiffParser<'a> {
    pub cursor: Cursor<&'a [u8]>,
}

impl<'a> VcdiffParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn new_at_offset(data: &'a [u8], offset: u64) -> Self {
        let mut cursor = Cursor::new(data);
        cursor.set_position(offset);
        Self { cursor }
    }

    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn is_eof(&self) -> bool {
        self.cursor.position() >= self.cursor.get_ref().len() as u64
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.cursor
            .read_exact(&mut buf)
            .map_err(|_| PatchError::CorruptedData)?;
        Ok(buf[0])
    }

    pub fn read_7bit_encoded_int(&mut self) -> Result<u64> {
        let mut num: u64 = 0;
        let mut bits: u8;
        loop {
            bits = self.read_u8()?;
            num = (num << 7) + (bits & 0x7f) as u64;
            if (bits & 0x80) == 0 {
                break;
            }
        }
        Ok(num)
    }

    pub fn skip(&mut self, amount: u64) -> Result<()> {
        self.cursor
            .seek(SeekFrom::Current(amount as i64))
            .map_err(|_| PatchError::CorruptedData)?;
        Ok(())
    }

    pub fn seek(&mut self, pos: u64) -> Result<()> {
        self.cursor.set_position(pos);
        Ok(())
    }
}

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

impl<'a> VcdiffParser<'a> {
    pub fn decode_window_header(&mut self) -> Result<WindowHeader> {
        let indicator = self.read_u8()?;
        let mut source_length = 0;
        let mut source_position = 0;

        if (indicator & (VCD_SOURCE | VCD_TARGET)) != 0 {
            source_length = self.read_7bit_encoded_int()?;
            source_position = self.read_7bit_encoded_int()?;
        }

        let delta_length = self.read_7bit_encoded_int()?;
        let target_window_length = self.read_7bit_encoded_int()?;
        let delta_indicator = self.read_u8()?;

        if delta_indicator != 0 {
            return Err(PatchError::Other(format!(
                "Unimplemented window header delta indicator: {}",
                delta_indicator
            )));
        }

        let add_run_data_length = self.read_7bit_encoded_int()?;
        let instructions_length = self.read_7bit_encoded_int()?;
        let addresses_length = self.read_7bit_encoded_int()?;

        let mut adler32 = None;
        if (indicator & VCD_ADLER32) != 0 {
            let mut buf = [0u8; 4];
            self.cursor
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

pub fn apply_patch(rom: &mut Vec<u8>, patch: &[u8]) -> Result<()> {
    let mut parser = VcdiffParser::new(patch);

    // Skip VCDIFF Header (4 bytes: Magic D6 C3 C4 + Version 00)
    parser.seek(4)?;

    let header_indicator = parser.read_u8()?;

    if (header_indicator & VCD_DECOMPRESS) != 0 {
        let secondary_compressor_id = parser.read_u8()?;
        if secondary_compressor_id != 0 {
            return Err(PatchError::Other(
                "Secondary decompressor not implemented".to_string(),
            ));
        }
    }

    if (header_indicator & VCD_CODETABLE) != 0 {
        let code_table_len = parser.read_7bit_encoded_int()?;
        if code_table_len != 0 {
            return Err(PatchError::Other(
                "Custom code table not implemented".to_string(),
            ));
        }
    }

    if (header_indicator & VCD_APPHEADER) != 0 {
        let app_data_len = parser.read_7bit_encoded_int()?;
        parser.skip(app_data_len)?;
    }

    let code_table = get_default_code_table();
    let mut cache = AddressCache::default();

    // Calculate final target size
    let header_end_offset = parser.position();
    let mut target_size = 0;
    while !parser.is_eof() {
        let win_header = parser.decode_window_header()?;
        target_size += win_header.target_window_length;
        parser.skip(
            win_header.add_run_data_length
                + win_header.instructions_length
                + win_header.addresses_length,
        )?;
    }

    // Create new target buffer
    let mut target = Vec::with_capacity(target_size as usize);
    // Keep source as read-only.
    let source = rom.clone();

    parser.seek(header_end_offset)?;

    let mut target_window_position: u64 = 0;

    while !parser.is_eof() {
        let window_start_pos = parser.position();
        let win_header = parser.decode_window_header()?;

        let add_run_data_offset = parser.position();
        let instructions_offset = add_run_data_offset + win_header.add_run_data_length;
        let addresses_offset = instructions_offset + win_header.instructions_length;

        let mut add_run_stream = VcdiffParser::new_at_offset(patch, add_run_data_offset);
        let mut inst_stream = VcdiffParser::new_at_offset(patch, instructions_offset);
        let mut addr_stream = VcdiffParser::new_at_offset(patch, addresses_offset);

        let mut add_run_data_index: u64 = 0;

        cache.reset();

        let target_window_start_index = target.len();

        while inst_stream.position() < addresses_offset {
            let instruction_index = inst_stream.read_u8()? as usize;

            for instruction in &code_table[instruction_index] {
                let mut size = instruction.size as u64;

                if size == 0 && instruction.inst_type != VCD_NOOP {
                    size = inst_stream.read_7bit_encoded_int()?;
                }

                match instruction.inst_type {
                    VCD_NOOP => continue,
                    VCD_ADD => {
                        let mut data = vec![0u8; size as usize];
                        add_run_stream
                            .cursor
                            .read_exact(&mut data)
                            .map_err(|_| PatchError::CorruptedData)?;
                        target.extend_from_slice(&data);
                        add_run_data_index += size;
                    }
                    VCD_RUN => {
                        let run_byte = add_run_stream.read_u8()?;
                        for _ in 0..size {
                            target.push(run_byte);
                        }
                        add_run_data_index += size;
                    }
                    VCD_COPY => {
                        let here_val = add_run_data_index + win_header.source_length;
                        let addr = decode_address(
                            &mut cache,
                            &mut addr_stream,
                            here_val,
                            instruction.mode,
                        )?;

                        let mut abs_addr: u64;
                        let use_source: bool;

                        if addr < win_header.source_length {
                            abs_addr = win_header.source_position + addr;
                            if (win_header.indicator & VCD_SOURCE) != 0 {
                                use_source = true;
                            } else if (win_header.indicator & VCD_TARGET) != 0 {
                                // "Target" mode implies decoding from target
                                abs_addr = win_header.source_position + addr;
                                use_source = false;
                            } else {
                                use_source = true; // Default
                            }
                        } else {
                            // Address in current target window
                            abs_addr = target_window_position + (addr - win_header.source_length);
                            use_source = false;
                        }

                        for _ in 0..size {
                            let byte = if use_source {
                                if (abs_addr as usize) < source.len() {
                                    source[abs_addr as usize]
                                } else {
                                    0 // Out of bounds source read
                                }
                            } else if (abs_addr as usize) < target.len() {
                                target[abs_addr as usize]
                            } else {
                                0 // Out of bounds target read (forward reference?)
                            };
                            target.push(byte);
                            abs_addr += 1;
                        }
                    }
                    _ => return Err(PatchError::CorruptedData),
                }
            }
        }

        let window_end_pos = parser.position()
            + win_header.add_run_data_length
            + win_header.instructions_length
            + win_header.addresses_length;

        parser.seek(window_end_pos)?;

        // Verify Delta Length
        if (window_end_pos - window_start_pos) != win_header.delta_length {
            // We can be strict here as we want valid patches
            return Err(PatchError::Other(format!(
                "VCDIFF window delta length mismatch: expected {}, got {}",
                win_header.delta_length,
                window_end_pos - window_start_pos
            )));
        }

        // Verify Adler32
        if let Some(expected_checksum) = win_header.adler32 {
            let window_data = &target[target_window_start_index..];
            let actual_checksum = adler32::compute(window_data);
            if actual_checksum != expected_checksum {
                return Err(PatchError::ChecksumMismatch {
                    expected: expected_checksum,
                    actual: actual_checksum,
                });
            }
        }

        target_window_position += win_header.target_window_length;
    }

    *rom = target;
    Ok(())
}

fn decode_address(
    cache: &mut AddressCache,
    stream: &mut VcdiffParser,
    here: u64,
    mode: u8,
) -> Result<u64> {
    let address: u64;

    if mode == VCD_MODE_SELF {
        address = stream.read_7bit_encoded_int()?;
    } else if mode == VCD_MODE_HERE {
        address = here - stream.read_7bit_encoded_int()?;
    } else if (mode as usize - 2) < 4 {
        // Near cache
        address = cache.get_near((mode as usize) - 2) + stream.read_7bit_encoded_int()?;
    } else {
        // Same cache
        let m = (mode as usize) - (2 + 4);
        address = cache.get_same(m * 256 + stream.read_u8()? as usize);
    }

    cache.update(address);
    Ok(address)
}
