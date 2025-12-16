use bzip2::Compression;
use bzip2::write::BzEncoder;
use stitchr_core::PatchFormat;
use stitchr_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};
use std::io::Write;

fn create_valid_bzip_block(data: &[u8]) -> Vec<u8> {
    let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn test_apply_multiple_records() {
    // 1. Copy 2 bytes from OLD (Diff 2, Extra 0)
    // 2. Insert 2 bytes (Diff 0, Extra 2)
    // Old: [A, B, C, D]
    // New: [A+d1, B+d2, X, Y]

    let old_rom = vec![10, 20, 30, 40];
    // We want Output: [11, 22, 100, 101]

    // Record 1: Diff 2, Extra 0, Seek 0
    // Diff data: [1, 2] -> 10+1=11, 20+2=22
    // Seek: 0

    // Record 2: Diff 0, Extra 2, Seek 0
    // Extra data: [100, 101]
    // Seek: 0

    let mut control_data = Vec::new();
    // Rec 1
    control_data.extend_from_slice(&2u64.to_le_bytes()); // diff
    control_data.extend_from_slice(&0u64.to_le_bytes()); // extra
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek

    // Rec 2
    control_data.extend_from_slice(&0u64.to_le_bytes()); // diff
    control_data.extend_from_slice(&2u64.to_le_bytes()); // extra
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek

    let control = create_valid_bzip_block(&control_data);
    let diff = create_valid_bzip_block(&[1, 2]);
    let extra = create_valid_bzip_block(&[100, 101]);

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&4u64.to_le_bytes()); // Patched Size

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&extra);

    let mut rom = old_rom.clone();
    let patcher = BdfPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom, vec![11, 22, 100, 101]);
}

#[test]
fn test_apply_seek_backwards() {
    // Read 1 byte, seek back 1, read same byte again
    let old_rom = vec![55, 66];

    // Rec 1: Diff 1, Extra 0, Seek -1
    // Rec 2: Diff 1, Extra 0, Seek 0

    let mut control_data = Vec::new();
    control_data.extend_from_slice(&1u64.to_le_bytes());
    control_data.extend_from_slice(&0u64.to_le_bytes());
    control_data.extend_from_slice(&((-1i64).to_le_bytes()));

    control_data.extend_from_slice(&1u64.to_le_bytes());
    control_data.extend_from_slice(&0u64.to_le_bytes());
    control_data.extend_from_slice(&0i64.to_le_bytes());

    let control = create_valid_bzip_block(&control_data);
    let diff = create_valid_bzip_block(&[0, 0]); // Add 0 to both
    let extra = create_valid_bzip_block(&[]);

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&2u64.to_le_bytes());

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&extra);

    let mut rom = old_rom.clone();
    let patcher = BdfPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    // Output should be [55, 55] because we read 55, seek back to 0, read 55 again
    assert_eq!(rom, vec![55, 55]);
}

#[test]
fn test_apply_seek_out_of_bounds() {
    // Reading past EOF should give 0
    let old_rom = vec![10];

    // Diff 2 bytes. 1st is in bounds, 2nd is out (should be 0)
    // Diff data: [5, 5]
    // 1st: 10 + 5 = 15
    // 2nd: 0 + 5 = 5

    let mut control_data = Vec::new();
    control_data.extend_from_slice(&2u64.to_le_bytes());
    control_data.extend_from_slice(&0u64.to_le_bytes());
    control_data.extend_from_slice(&0i64.to_le_bytes());

    let control = create_valid_bzip_block(&control_data);
    let diff = create_valid_bzip_block(&[5, 5]);
    let extra = create_valid_bzip_block(&[]);

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&2u64.to_le_bytes());

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&extra);

    let mut rom = old_rom.clone();
    BdfPatcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom, vec![15, 5]);
}
