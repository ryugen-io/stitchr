use super::constants::*;
use super::helpers::{parse_header, parse_record};
use stitchr_core::{PatchError, Result};

pub fn apply(rom: &[u8], patch: &[u8]) -> Result<Vec<u8>> {
    let (header, mut offset) = parse_header(patch)?;

    // Limit target size to prevent ASAN crashes
    const MAX_TARGET_SIZE: u32 = 512 * 1024 * 1024;
    if header.target_size > MAX_TARGET_SIZE {
        return Err(PatchError::InvalidFormat(format!(
            "Target size too large: {} (max {})",
            header.target_size, MAX_TARGET_SIZE
        )));
    }

    let mut output = Vec::new();
    output
        .try_reserve_exact(header.target_size as usize)
        .map_err(|_| PatchError::Other("Failed to allocate memory for target ROM".to_string()))?;
    output.resize(header.target_size as usize, 0);
    let copy_len = rom.len().min(output.len());
    output[..copy_len].copy_from_slice(&rom[..copy_len]);

    while offset < patch.len() {
        let record = parse_record(patch, offset)?;
        offset += RECORD_SIZE;

        let block_end = (record.offset as usize + BLOCK_SIZE).min(output.len());
        let block_len = block_end - record.offset as usize;

        if record.offset as usize >= output.len() {
            return Err(PatchError::OutOfBounds {
                offset: record.offset as usize,
                rom_size: output.len(),
            });
        }

        for i in 0..block_len {
            let rom_idx = record.offset as usize + i;
            let src_byte = if rom_idx < rom.len() { rom[rom_idx] } else { 0 };
            output[rom_idx] = src_byte ^ record.xor_data[i];
        }
    }

    Ok(output)
}

pub fn verify(rom: &[u8], patch: &[u8]) -> Result<()> {
    let (header, mut offset) = parse_header(patch)?;

    if rom.len() != header.source_size as usize {
        return Err(PatchError::SizeMismatch {
            expected: header.source_size as usize,
            actual: rom.len(),
        });
    }

    while offset < patch.len() {
        let record = parse_record(patch, offset)?;
        offset += RECORD_SIZE;

        if record.offset as usize + BLOCK_SIZE > rom.len() {
            continue;
        }

        let block = &rom[record.offset as usize..record.offset as usize + BLOCK_SIZE];
        let calculated_crc = crc16::State::<crc16::CCITT_FALSE>::calculate(block);

        if calculated_crc != record.source_crc16 {
            return Err(PatchError::ChecksumMismatch {
                expected: record.source_crc16 as u32,
                actual: calculated_crc as u32,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_patch(target_size: u32, offset: u32, xor_data: &[u8]) -> Vec<u8> {
        let mut patch = Vec::new();
        patch.extend_from_slice(MAGIC);
        patch.extend_from_slice(&512u32.to_le_bytes());
        patch.extend_from_slice(&target_size.to_le_bytes());
        patch.extend_from_slice(&offset.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        patch.extend_from_slice(&0u16.to_le_bytes());
        let mut full_xor = xor_data.to_vec();
        full_xor.resize(BLOCK_SIZE, 0);
        patch.extend_from_slice(&full_xor);
        patch
    }

    #[test]
    fn test_apply_with_xor() {
        let xor_data = vec![0xFF; 4];
        let patch = make_patch(512, 0x10, &xor_data);
        let rom = vec![0xAA; 512];

        let output = apply(&rom, &patch).unwrap();
        assert_eq!(output.len(), 512);
        assert_eq!(&output[0x10..0x14], &[0x55, 0x55, 0x55, 0x55]);
    }
}
