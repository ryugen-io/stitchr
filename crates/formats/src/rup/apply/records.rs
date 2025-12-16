//! RUP XOR record handling

use super::super::constants::*;
use super::super::varint::decode_vlv;
use stitchr_core::{PatchError, Result};

pub struct XorRecord {
    pub offset: u64,
    pub data: Vec<u8>,
}

/// Collect all XOR records for current file
pub fn collect_records(patch: &[u8], mut offset: usize) -> Result<Vec<XorRecord>> {
    let mut records = Vec::new();
    while offset < patch.len() {
        let command = patch[offset];
        offset += 1;
        if command == COMMAND_XOR_RECORD {
            let (rec_offset, consumed) = decode_vlv(&patch[offset..])?;
            offset += consumed;
            let (len, consumed) = decode_vlv(&patch[offset..])?;
            offset += consumed;
            if offset + len as usize > patch.len() {
                return Err(PatchError::UnexpectedEof("XOR data".to_string()));
            }
            let data = patch[offset..offset + len as usize].to_vec();
            offset += len as usize;
            records.push(XorRecord {
                offset: rec_offset,
                data,
            });
        } else {
            break;
        }
    }
    Ok(records)
}

/// Apply XOR records to ROM
pub fn apply_xor_records(rom: &mut [u8], records: &[XorRecord]) -> Result<()> {
    for record in records {
        let offset = record.offset as usize;
        for (i, &byte) in record.data.iter().enumerate() {
            let pos = offset + i;
            if pos < rom.len() {
                rom[pos] ^= byte;
            }
        }
    }
    Ok(())
}
