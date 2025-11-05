//! Tests for BPS patch application

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

#[test]
fn test_apply_source_read() {
    // SOURCE_READ: Copy from ROM at current output position
    let mut rom = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

    // Build BPS patch inline
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x85); // source_size = 5 (varint)
    patch.push(0x83); // target_size = 3 (varint)
    patch.push(0x80); // metadata_size = 0 (varint)
    patch.push(0x88); // SOURCE_READ length=3: ((3-1)<<2) | 0 = 8
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_target_read() {
    // TARGET_READ: Copy bytes from patch data
    let mut rom = vec![0x00; 10];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x8A); // source_size = 10 (varint)
    patch.push(0x83); // target_size = 3 (varint)
    patch.push(0x80); // metadata_size = 0
    patch.push(0x89); // TARGET_READ length=3: ((3-1)<<2) | 1 = 9
    patch.extend_from_slice(&[0xFF, 0xEE, 0xDD]); // data
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_source_copy() {
    // SOURCE_COPY: Copy from ROM at relative offset
    let mut rom = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x86); // source_size = 6 (varint)
    patch.push(0x82); // target_size = 2 (varint)
    patch.push(0x80); // metadata_size = 0
    patch.push(0x86); // SOURCE_COPY length=2: ((2-1)<<2) | 2 = 6
    patch.push(0x84); // relative_offset: (2<<1) | 0 = 4
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_target_copy_rle() {
    // TARGET_COPY: RLE-style overlapping copy
    let mut rom = vec![0x00; 10];

    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    patch.push(0x8A); // source_size = 10 (varint)
    patch.push(0x86); // target_size = 6 (varint)
    patch.push(0x80); // metadata_size = 0
    patch.push(0x81); // TARGET_READ length=1: ((1-1)<<2) | 1 = 1
    patch.push(0xFF); // data byte to repeat
    patch.push(0x93); // TARGET_COPY length=5: ((5-1)<<2) | 3 = 19
    patch.push(0x83); // relative_offset: (-1 encoded as (1<<1)|1 = 3)
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

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
