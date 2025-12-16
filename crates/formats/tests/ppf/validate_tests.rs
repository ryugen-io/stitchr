//! PPF validation tests.

use stitchr_core::{PatchError, PatchFormat};
use stitchr_formats::ppf::PpfPatcher;

#[test]

fn test_validate_valid_ppf3() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02); // Encoding

    patch_data.extend_from_slice(&[0u8; 50]); // Description

    patch_data.push(0x00); // Image Type

    patch_data.push(0x00); // Block Check (False)

    patch_data.push(0x00); // Undo Data (False)

    patch_data.push(0x00); // Dummy

    // Record

    patch_data.extend_from_slice(&0u64.to_le_bytes());

    patch_data.push(0x01);

    patch_data.push(0xAA);

    assert!(PpfPatcher::validate(&patch_data).is_ok());
}

#[test]

fn test_validate_truncated_offset() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02);

    patch_data.extend_from_slice(&[0u8; 50]);

    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);

    patch_data.extend_from_slice(&[0u8; 4]);

    assert!(matches!(
        PpfPatcher::validate(&patch_data),
        Err(PatchError::CorruptedData)
    ));
}

#[test]

fn test_validate_invalid_magic() {
    let patch_data = b"NOTPPF";

    assert!(matches!(
        PpfPatcher::validate(patch_data),
        Err(PatchError::InvalidFormat(_))
    ));
}

#[test]

fn test_validate_truncated_data() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02);

    patch_data.extend_from_slice(&[0u8; 50]);

    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);

    patch_data.extend_from_slice(&0u64.to_le_bytes());

    patch_data.push(5); // Length 5

    patch_data.push(0xAA); // Only 1 byte of data

    assert!(matches!(
        PpfPatcher::validate(&patch_data),
        Err(PatchError::CorruptedData)
    ));
}

#[test]

fn test_validate_eof_in_header() {
    let patch_data = b"PPF30"; // Missing everything else

    assert!(matches!(
        PpfPatcher::validate(patch_data),
        Err(PatchError::CorruptedData)
    ));
}

#[test]

fn test_validate_block_check_bounds() {
    let mut patch_data = Vec::new();

    patch_data.extend_from_slice(b"PPF30");

    patch_data.push(0x02);

    patch_data.extend_from_slice(&[0u8; 50]);

    patch_data.push(0x00); // Image

    patch_data.push(0x01); // Block Check TRUE

    patch_data.push(0x00);
    patch_data.push(0x00);

    // Only provide 10 bytes of block check, need 1024

    patch_data.extend_from_slice(&[0xFF; 10]);

    assert!(matches!(
        PpfPatcher::validate(&patch_data),
        Err(PatchError::CorruptedData)
    ));
}
