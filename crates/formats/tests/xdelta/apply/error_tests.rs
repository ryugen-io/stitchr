//! xdelta error handling tests

use stitchr_core::{PatchError, PatchFormat};
use stitchr_formats::xdelta::XdeltaPatcher;

#[test]
fn test_xdelta_invalid_magic() {
    let patch = b"NOT_XDELTA";
    let mut rom = vec![0u8; 10];
    let patcher = XdeltaPatcher;
    assert!(matches!(
        patcher.apply(&mut rom, patch),
        Err(PatchError::InvalidMagic { .. })
    ));
}

#[test]
fn test_xdelta_unimplemented_secondary_compressor() {
    // VCDIFF Header: D6 C3 C4 00
    // Indicator: VCD_DECOMPRESS (0x01)
    let mut patch = vec![0xD6, 0xC3, 0xC4, 0x00];
    patch.push(0x01); // Header Indicator
    patch.push(0x01); // Compressor ID != 0

    let mut rom = vec![0u8; 10];
    let patcher = XdeltaPatcher;
    let result = patcher.apply(&mut rom, &patch);
    // Should return Error::Other("Secondary decompressor not implemented")
    match result {
        Err(PatchError::Other(msg)) => assert_eq!(msg, "Secondary decompressor not implemented"),
        _ => panic!("Expected Other error, got {:?}", result),
    }
}
