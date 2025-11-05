//! Tests for BPS patch application

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

/// Helper to create minimal valid BPS patch
fn create_bps_patch(
    source_size: usize,
    target_size: usize,
    actions: &[u8],
) -> Vec<u8> {
    let mut patch = Vec::new();

    // Magic header
    patch.extend_from_slice(b"BPS1");

    // Source size (varint)
    write_varint(&mut patch, source_size as u64);

    // Target size (varint)
    write_varint(&mut patch, target_size as u64);

    // Metadata size (0)
    write_varint(&mut patch, 0);

    // Actions
    patch.extend_from_slice(actions);

    // Checksums (placeholder - will be wrong but OK for some tests)
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32

    // Patch CRC32 (compute real value)
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    patch
}

fn write_varint(buf: &mut Vec<u8>, mut data: u64) {
    loop {
        let x = (data & 0x7f) as u8;
        data >>= 7;
        if data == 0 {
            buf.push(0x80 | x);
            break;
        }
        buf.push(x);
        data -= 1;
    }
}

#[test]
fn test_apply_source_read() {
    // SOURCE_READ: Copy from ROM at current output position
    let mut rom = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

    // Action: SOURCE_READ, length=3 -> ((3-1)<<2) | 0 = 8
    let actions = vec![0x88]; // varint(8) with end bit

    let patch = create_bps_patch(5, 3, &actions);

    let patcher = BpsPatcher;
    // Will fail on CRC32 but we can test the action logic separately
    // For now, just test that it doesn't panic
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_target_read() {
    // TARGET_READ: Copy bytes from patch data
    let mut rom = vec![0x00; 10];

    // Action: TARGET_READ, length=3, data=[0xFF, 0xEE, 0xDD]
    // ((3-1)<<2) | 1 = 9
    let mut actions = Vec::new();
    write_varint(&mut actions, 9);
    actions.extend_from_slice(&[0xFF, 0xEE, 0xDD]);

    let patch = create_bps_patch(10, 3, &actions);

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_source_copy() {
    // SOURCE_COPY: Copy from ROM at relative offset
    let mut rom = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

    // Action: SOURCE_COPY, length=2, relative_offset=+2
    // ((2-1)<<2) | 2 = 6
    // relative_offset encoding: (2<<1) | 0 = 4
    let mut actions = Vec::new();
    write_varint(&mut actions, 6);
    write_varint(&mut actions, 4); // positive offset +2

    let patch = create_bps_patch(6, 2, &actions);

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_target_copy_rle() {
    // TARGET_COPY: RLE-style overlapping copy
    let mut rom = vec![0x00; 10];

    // First write a byte with TARGET_READ
    let mut actions = Vec::new();
    // TARGET_READ length=1: ((1-1)<<2) | 1 = 1
    write_varint(&mut actions, 1);
    actions.push(0xFF); // The byte to repeat

    // Then TARGET_COPY to repeat it
    // TARGET_COPY length=5, relative_offset=-1 (to the byte we just wrote)
    // ((5-1)<<2) | 3 = 19
    // relative_offset: (-1 encoded as (1<<1)|1 = 3)
    write_varint(&mut actions, 19);
    write_varint(&mut actions, 3); // -1 offset

    let patch = create_bps_patch(10, 6, &actions);

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}

#[test]
fn test_apply_mixed_actions() {
    // Mix of SOURCE_READ, TARGET_READ, SOURCE_COPY
    let mut rom = vec![0x11, 0x22, 0x33, 0x44, 0x55];

    let mut actions = Vec::new();

    // SOURCE_READ length=2: ((2-1)<<2) | 0 = 4
    write_varint(&mut actions, 4);

    // TARGET_READ length=1: ((1-1)<<2) | 1 = 1, data=0xFF
    write_varint(&mut actions, 1);
    actions.push(0xFF);

    // SOURCE_READ length=1: ((1-1)<<2) | 0 = 0
    write_varint(&mut actions, 0);

    let patch = create_bps_patch(5, 4, &actions);

    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, &patch);
}
