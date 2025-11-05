//! Basic UPS application tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ups::UpsPatcher;

#[test]
fn test_can_handle() {
    assert!(UpsPatcher::can_handle(b"UPS1"));
    assert!(UpsPatcher::can_handle(b"UPS1\x00\x00\x00\x00\x00\x00\x00\x00\x00"));
    assert!(!UpsPatcher::can_handle(b"PATCH"));
    assert!(!UpsPatcher::can_handle(b"UPS"));
    assert!(!UpsPatcher::can_handle(b""));
}

#[test]
fn test_apply_simple_xor() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x8A); // Output size = 10
    patch.push(0x80); // Relative offset 0
    patch.push(0xFF); // XOR with 0xFF
    patch.push(0x00); // Terminator

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = input_rom.clone();
    output_rom[0] = 0xFF;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom[0], 0xFF);
    assert_eq!(rom[1], 0x00);
}

#[test]
fn test_apply_empty_patch() {
    let rom = vec![0x12, 0x34, 0x56];
    let original = rom.clone();

    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x83); // Input size = 3
    patch.push(0x83); // Output size = 3
    patch.push(0x80); // Offset 0
    patch.push(0x00); // XOR terminator (no data)

    let input_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let output_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom_mut = rom;
    patcher.apply(&mut rom_mut, &patch).unwrap();
    assert_eq!(rom_mut, original);
}
