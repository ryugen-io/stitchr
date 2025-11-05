//! UPS ROM resize tests

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ups::UpsPatcher;

#[test]
fn test_apply_rom_resize() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"UPS1");
    patch.push(0x8A); // Input size = 10
    patch.push(0x94); // Output size = 20

    patch.push(0x8F); // Relative offset 15
    patch.push(0xCC);
    patch.push(0x00);

    let input_rom = vec![0u8; 10];
    let input_crc = crc32fast::hash(&input_rom);
    patch.extend_from_slice(&input_crc.to_le_bytes());

    let mut output_rom = vec![0u8; 20];
    output_rom[15] = 0xCC;
    let output_crc = crc32fast::hash(&output_rom);
    patch.extend_from_slice(&output_crc.to_le_bytes());

    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = UpsPatcher;
    let mut rom = vec![0u8; 10];
    assert!(patcher.apply(&mut rom, &patch).is_ok());
    assert_eq!(rom.len(), 20);
    assert_eq!(rom[15], 0xCC);
}
