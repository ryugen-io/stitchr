//! UPS XOR operation tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ups::UpsPatcher;

#[test]
fn test_apply_multiple_xor_records() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10

    // First XOR: at offset 0, change to 0xAA
    patch.push(0x80); // Relative offset 0
    patch.push(0xAA); // XOR data
    patch.push(0x00); // Terminator

    // Second XOR: at offset 4 (relative offset from pos 2 = 2)
    patch.push(0x82); // Relative offset 2
    patch.push(0xBB); // XOR data
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom.clone();
    output_rom[0] = 0xAA;
    output_rom[4] = 0xBB;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xAA);
    assert_eq!(rom[4], 0xBB);
}

#[test]
fn test_apply_multi_byte_xor() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10

    patch.push(0x80); // Relative offset 0
    patch.push(0xFF);
    patch.push(0xEE);
    patch.push(0xDD);
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom.clone();
    output_rom[0] = 0xFF;
    output_rom[1] = 0xEE;
    output_rom[2] = 0xDD;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xFF);
    assert_eq!(rom[1], 0xEE);
    assert_eq!(rom[2], 0xDD);
}
