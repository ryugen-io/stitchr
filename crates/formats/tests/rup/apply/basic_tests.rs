//! Basic RUP application tests

use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_can_handle() {
    assert!(RupPatcher::can_handle(b"NINJA2"));
    assert!(RupPatcher::can_handle(
        b"NINJA2\x00\x00\x00\x00\x00\x00\x00\x00"
    ));
    assert!(!RupPatcher::can_handle(b"PATCH"));
    assert!(!RupPatcher::can_handle(b"NINJA"));
    assert!(!RupPatcher::can_handle(b""));
}

#[test]
fn test_apply_real_patch() {
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");
    let rom_path = std::path::Path::new("../../test_files/rup/rom.sfc");

    if !patch_path.exists() || !rom_path.exists() {
        println!("Skipping test: files not found in test_files/rup/");
        return;
    }

    let patch = fs::read(patch_path).expect("Failed to read RUP patch");
    let mut rom = fs::read(rom_path).expect("Failed to read ROM");

    let patcher = RupPatcher;
    let result = patcher.apply(&mut rom, &patch);

    assert!(result.is_ok(), "Patch application should succeed");

    // Verify output size matches expected (1572864 bytes)
    assert_eq!(rom.len(), 1572864);

    // Verify output CRC32 matches expected (headered)
    let output_crc = crc32fast::hash(&rom);
    assert_eq!(output_crc, 0xda833bce);
}

#[test]
fn test_apply_preserves_rom_on_error() {
    let invalid_patch = b"NINJA2";
    let mut rom = vec![0xAA; 100];
    let original = rom.clone();

    let patcher = RupPatcher;
    let result = patcher.apply(&mut rom, invalid_patch);

    assert!(result.is_err());
    assert_eq!(rom, original, "ROM should be unchanged on error");
}

#[test]
fn test_validate_before_apply() {
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");

    if !patch_path.exists() {
        println!("Skipping test: files not found in test_files/rup/");
        return;
    }

    let patch = fs::read(patch_path).expect("Failed to read RUP patch");

    assert!(
        RupPatcher::validate(&patch).is_ok(),
        "Patch should be valid before apply"
    );
}
