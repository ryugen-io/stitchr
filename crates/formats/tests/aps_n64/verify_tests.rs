//! Tests for APS N64 checksum verification

use stitchr_core::PatchFormat;
use stitchr_formats::aps::n64::ApsN64Patcher;

const N64_CART_ID_OFFSET: usize = 0x3C;
const N64_CRC_OFFSET: usize = 0x10;

fn make_header_with_n64_info(cart_id: &[u8; 3], crc: &[u8; 8]) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01); // header_type = N64
    patch.push(0x00); // encoding_method
    patch.extend_from_slice(&[0u8; 50]); // description
    patch.push(0x01); // original_format
    patch.extend_from_slice(cart_id); // cart_id (3 bytes)
    patch.extend_from_slice(crc); // crc (8 bytes)
    patch.extend_from_slice(&[0u8; 5]); // padding
    patch.extend_from_slice(&512u32.to_le_bytes()); // output_size
    patch
}

fn make_rom_with_n64_info(cart_id: &[u8; 3], crc: &[u8; 8]) -> Vec<u8> {
    let mut rom = vec![0u8; 256];
    rom[N64_CART_ID_OFFSET..N64_CART_ID_OFFSET + 3].copy_from_slice(cart_id);
    rom[N64_CRC_OFFSET..N64_CRC_OFFSET + 8].copy_from_slice(crc);
    rom
}

#[test]
fn test_verify_input_rom() {
    let cart_id = [b'N', b'T', b'S'];
    let crc = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

    let rom = make_rom_with_n64_info(&cart_id, &crc);
    let patch = make_header_with_n64_info(&cart_id, &crc);

    assert!(ApsN64Patcher::verify(&rom, &patch, None).is_ok());
}

#[test]
fn test_verify_wrong_cart_id() {
    let cart_id = [b'N', b'T', b'S'];
    let crc = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

    let rom = make_rom_with_n64_info(&cart_id, &crc);

    let wrong_cart_id = [b'W', b'R', b'G'];
    let patch = make_header_with_n64_info(&wrong_cart_id, &crc);

    assert!(ApsN64Patcher::verify(&rom, &patch, None).is_err());
}

#[test]
fn test_verify_wrong_crc() {
    let cart_id = [b'N', b'T', b'S'];
    let crc = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

    let rom = make_rom_with_n64_info(&cart_id, &crc);

    let wrong_crc = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let patch = make_header_with_n64_info(&cart_id, &wrong_crc);

    assert!(ApsN64Patcher::verify(&rom, &patch, None).is_err());
}

#[test]
fn test_verify_rom_too_small() {
    let cart_id = [b'N', b'T', b'S'];
    let crc = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];

    let rom = vec![0u8; 16]; // Too small - minimum is 0x10 (16) + 8 (CRC_LEN) = 24 bytes
    let patch = make_header_with_n64_info(&cart_id, &crc);

    assert!(ApsN64Patcher::verify(&rom, &patch, None).is_err());
}
