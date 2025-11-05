//! UPS checksum validation tests with real patch file

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ups::UpsPatcher;
use std::fs;

#[test]
fn test_patch_file_integrity() {
    let patch = fs::read("../../test_files/ups/test.ups").expect("Failed to read UPS patch");

    // UPS validate() checks patch CRC32
    assert!(
        UpsPatcher::validate(&patch).is_ok(),
        "Patch CRC32 validation should succeed for valid UPS patch"
    );
}

#[test]
fn test_rom_file_exists() {
    let rom_path = "../../test_files/ups/test.rom.gba";
    assert!(
        fs::metadata(rom_path).is_ok(),
        "Test ROM file should exist at {}",
        rom_path
    );
}

#[test]
fn test_mother3_patch() {
    // This test verifies we can read and validate the Mother 3 UPS patch
    let patch = fs::read("../../test_files/ups/test.ups").expect("Failed to read UPS patch");

    // Validate patch integrity
    assert!(
        UpsPatcher::validate(&patch).is_ok(),
        "Mother 3 UPS patch should pass CRC32 validation"
    );

    // Extract metadata
    let metadata = UpsPatcher::metadata(&patch).expect("Failed to extract metadata");

    assert!(
        metadata.source_size.is_some(),
        "Metadata should include source size"
    );
    assert!(
        metadata.target_size.is_some(),
        "Metadata should include target size"
    );
    assert!(
        metadata.source_checksum.is_some(),
        "Metadata should include source checksum"
    );
    assert!(
        metadata.target_checksum.is_some(),
        "Metadata should include target checksum"
    );
}

#[test]
fn test_apply_and_verify() {
    let patch = fs::read("../../test_files/ups/test.ups").expect("Failed to read UPS patch");
    let mut rom = fs::read("../../test_files/ups/test.rom.gba").expect("Failed to read ROM");

    let patcher = UpsPatcher;
    assert!(
        patcher.apply(&mut rom, &patch).is_ok(),
        "Should successfully apply UPS patch"
    );

    // Verify the result matches target checksum
    assert!(
        UpsPatcher::verify(&[], &patch, Some(&rom)).is_ok(),
        "Output ROM should match target checksum"
    );
}
