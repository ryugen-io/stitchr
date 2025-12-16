//! BPS mixed operation tests

use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;

#[test]
fn test_apply_mixed_actions() {
    // Mix of SOURCE_READ, TARGET_READ, SOURCE_COPY
    let mut rom = vec![0x11, 0x22, 0x33, 0x44, 0x55];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x85); // source_size = 5 (varint)
    patch.push(0x84); // target_size = 4 (varint)
    patch.push(0x80); // metadata_size = 0
    patch.push(0x84); // SOURCE_READ length=2: ((2-1)<<2) | 0 = 4
    patch.push(0x81); // TARGET_READ length=1: ((1-1)<<2) | 1 = 1
    patch.push(0xFF); // data
    patch.push(0x80); // SOURCE_READ length=1: ((1-1)<<2) | 0 = 0
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_empty_patch() {
    // Patch that copies source to target unchanged using SOURCE_READ
    let mut rom = vec![0x12, 0x34, 0x56];
    let original = rom.clone();

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x83); // source_size = 3 (varint)
    patch.push(0x83); // target_size = 3 (varint)
    patch.push(0x80); // metadata_size = 0 (varint)
    patch.push(0x88); // SOURCE_READ length=3: ((3-1)<<2) | 0 = 8
    let source_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&source_crc.to_le_bytes());
    let target_crc = crc32fast::hash(&rom);
    patch.extend_from_slice(&target_crc.to_le_bytes());
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom, original);
}
