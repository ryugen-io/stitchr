//! VCDIFF// Checked via compilation logic

use crate::xdelta::{
    address_cache::{AddressCache, decode_address},
    checksum::adler32,
    code_table::get_default_code_table,
    constants::{
        VCD_ADD, VCD_APPHEADER, VCD_CODETABLE, VCD_COPY, VCD_DECOMPRESS, VCD_NOOP, VCD_RUN,
        VCD_SOURCE, VCD_TARGET,
    },
    headers::{WindowHeader, calculate_target_size},
    parser::VcdiffParser,
};
use rom_patcher_core::{PatchError, Result};
use std::io::Read;

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
    let target_size = calculate_target_size(patch, header_end_offset)?;

    // Create new target buffer
    let mut target = Vec::with_capacity(target_size as usize);
    // Keep source as read-only.
    let source = rom.clone();

    // Reset parser to after header (calculate_target_size uses its own parser)
    parser.seek(header_end_offset)?;

    let mut target_window_position: u64 = 0;

    while !parser.is_eof() {
        let window_start_pos = parser.position();
        let win_header = WindowHeader::decode(&mut parser)?;

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
            return Err(PatchError::Other(format!(
                "VCDIFF window delta length mismatch: expected {}, got {}",
                win_header.delta_length,
                window_end_pos - window_start_pos
            )));
        }

        // Verify Adler32
        if let Some(expected_checksum) = win_header.adler32 {
            let window_data = &target[target_window_start_index..];
            let actual_checksum = adler32(window_data);
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
