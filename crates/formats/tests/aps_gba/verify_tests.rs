//! Tests for APS GBA checksum verification

use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;

const BLOCK_SIZE: usize = 0x10000; // 64KB

fn make_header(source_size: u32, target_size: u32) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&source_size.to_le_bytes());
    patch.extend_from_slice(&target_size.to_le_bytes());
    patch
}

fn calculate_crc16(data: &[u8]) -> u16 {
    crc16::State::<crc16::CCITT_FALSE>::calculate(data)
}

#[test]
fn test_verify_valid_rom() {
    let rom = vec![0xAA; 512];
    let block = &rom[0..BLOCK_SIZE.min(rom.len())];
    let mut full_block = vec![0xAA; BLOCK_SIZE];
    full_block[..block.len()].copy_from_slice(block);

    let source_crc = calculate_crc16(&full_block);

    let mut patch = make_header(512, 512);
    patch.extend_from_slice(&0x00u32.to_le_bytes()); // offset = 0
    patch.extend_from_slice(&source_crc.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes()); // target_crc (not used in verify)
    patch.extend_from_slice(&vec![0; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::verify(&rom, &patch, None).is_ok());
}

#[test]
fn test_verify_wrong_rom_size() {
    let rom = vec![0xAA; 512];

    let mut patch = make_header(1024, 1024); // Expecting 1024 bytes
    patch.extend_from_slice(&0x00u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::verify(&rom, &patch, None).is_err());
}

#[test]
fn test_verify_wrong_crc() {
    let rom = vec![0xAA; BLOCK_SIZE + 512]; // Make ROM large enough to contain the block

    let mut patch = make_header((BLOCK_SIZE + 512) as u32, (BLOCK_SIZE + 512) as u32);
    patch.extend_from_slice(&0x00u32.to_le_bytes());
    patch.extend_from_slice(&0xFFFFu16.to_le_bytes()); // Wrong CRC
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::verify(&rom, &patch, None).is_err());
}

#[test]
fn test_verify_multiple_records() {
    let rom = vec![0xBB; BLOCK_SIZE * 2];

    let block1 = &rom[0..BLOCK_SIZE];
    let source_crc1 = calculate_crc16(block1);

    let block2 = &rom[BLOCK_SIZE..BLOCK_SIZE * 2];
    let source_crc2 = calculate_crc16(block2);

    let mut patch = make_header((BLOCK_SIZE * 2) as u32, (BLOCK_SIZE * 2) as u32);

    // First record
    patch.extend_from_slice(&0x00u32.to_le_bytes());
    patch.extend_from_slice(&source_crc1.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0; BLOCK_SIZE]);

    // Second record
    patch.extend_from_slice(&(BLOCK_SIZE as u32).to_le_bytes());
    patch.extend_from_slice(&source_crc2.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0; BLOCK_SIZE]);

    assert!(ApsGbaPatcher::verify(&rom, &patch, None).is_ok());
}
