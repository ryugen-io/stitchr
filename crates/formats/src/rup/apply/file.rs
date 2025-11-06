//! RUP file metadata parsing

use super::records::{collect_records, XorRecord};
use super::super::varint::decode_vlv;
use rom_patcher_core::{PatchError, Result};

pub struct FileMeta {
    pub source_size: u64,
    pub target_size: u64,
    pub source_md5: [u8; 16],
    pub target_md5: [u8; 16],
    pub overflow_mode: Option<u8>,
    pub overflow_data: Vec<u8>,
    pub records: Vec<XorRecord>,
}

/// Parse file metadata from OPEN_NEW_FILE command
pub fn parse_file_metadata(patch: &[u8], mut offset: usize) -> Result<(FileMeta, usize)> {
    let (name_len, consumed) = decode_vlv(&patch[offset..])?;
    offset += consumed;

    if offset + name_len as usize > patch.len() {
        return Err(PatchError::UnexpectedEof("file name".to_string()));
    }
    offset += name_len as usize;

    if offset >= patch.len() {
        return Err(PatchError::UnexpectedEof("rom type".to_string()));
    }
    let rom_type = patch[offset];
    offset += 1;

    use super::super::constants::*;
    match rom_type {
        ROM_TYPE_RAW | ROM_TYPE_NES | ROM_TYPE_FDS | ROM_TYPE_SNES | ROM_TYPE_N64
        | ROM_TYPE_GB | ROM_TYPE_SMS | ROM_TYPE_MEGA | ROM_TYPE_PCE | ROM_TYPE_LYNX => {}
        _ => {
            return Err(PatchError::InvalidFormat(format!(
                "Invalid ROM type: {}",
                rom_type
            )))
        }
    }

    let (source_size, consumed) = decode_vlv(&patch[offset..])?;
    offset += consumed;
    let (target_size, consumed) = decode_vlv(&patch[offset..])?;
    offset += consumed;

    if offset + 32 > patch.len() {
        return Err(PatchError::UnexpectedEof("MD5 hashes".to_string()));
    }

    let mut source_md5 = [0u8; 16];
    let mut target_md5 = [0u8; 16];
    source_md5.copy_from_slice(&patch[offset..offset + 16]);
    target_md5.copy_from_slice(&patch[offset + 16..offset + 32]);
    offset += 32;

    let (overflow_mode, overflow_data) = if source_size != target_size {
        let mode = patch[offset];
        offset += 1;
        let (data_len, consumed) = decode_vlv(&patch[offset..])?;
        offset += consumed;
        if offset + data_len as usize > patch.len() {
            return Err(PatchError::UnexpectedEof("overflow data".to_string()));
        }
        let data = patch[offset..offset + data_len as usize].to_vec();
        offset += data_len as usize;
        (Some(mode), data)
    } else {
        (None, vec![])
    };

    let records = collect_records(patch, offset)?;
    Ok((
        FileMeta {
            source_size,
            target_size,
            source_md5,
            target_md5,
            overflow_mode,
            overflow_data,
            records,
        },
        offset,
    ))
}
