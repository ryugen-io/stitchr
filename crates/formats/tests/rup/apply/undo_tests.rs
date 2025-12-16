//! RUP bidirectional/undo tests

use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;
use std::fs;

#[test]
fn test_undo_capability() {
    // RUP is bidirectional - same patch can apply forward or undo
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");
    let rom_path = std::path::Path::new("../../test_files/rup/rom.sfc");

    if !patch_path.exists() || !rom_path.exists() {
        println!("Skipping test: files not found in test_files/rup/");
        return;
    }

    let patch = fs::read(patch_path).expect("Failed to read RUP patch");
    let mut rom = fs::read(rom_path).expect("Failed to read ROM");
    let original_rom = rom.clone();

    let patcher = RupPatcher;

    // Apply forward
    patcher
        .apply(&mut rom, &patch)
        .expect("Forward apply should succeed");
    let patched_rom = rom.clone();

    // Apply again (should undo)
    patcher
        .apply(&mut rom, &patch)
        .expect("Undo apply should succeed");

    assert_eq!(
        rom, original_rom,
        "Second apply should restore original ROM"
    );

    // Apply third time (forward again)
    patcher
        .apply(&mut rom, &patch)
        .expect("Third apply should succeed");

    assert_eq!(rom, patched_rom, "Third apply should match first result");
}

#[test]
fn test_verify_with_target() {
    let patch_path = std::path::Path::new("../../test_files/rup/test.rup");
    let rom_path = std::path::Path::new("../../test_files/rup/rom.sfc");

    if !patch_path.exists() || !rom_path.exists() {
        println!("Skipping test: files not found in test_files/rup/");
        return;
    }

    let patch = fs::read(patch_path).expect("Failed to read RUP patch");
    let mut source = fs::read(rom_path).expect("Failed to read ROM");

    let patcher = RupPatcher;
    patcher
        .apply(&mut source, &patch)
        .expect("Apply should succeed");

    let target = source.clone();
    let original = fs::read(rom_path).expect("Failed to read ROM");

    // Verify target from source
    let result = RupPatcher::verify(&original, &patch, Some(&target));
    assert!(result.is_ok(), "Target verification should succeed");
}
