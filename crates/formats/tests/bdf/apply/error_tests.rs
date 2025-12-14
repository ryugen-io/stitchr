use bzip2::Compression;
use bzip2::write::BzEncoder;
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};
use std::io::Write;

fn create_valid_bzip_block(data: &[u8]) -> Vec<u8> {
    let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

#[test]
fn test_apply_truncated_control_block() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    let control = create_valid_bzip_block(&[0; 10]); // Real data
    patch.extend_from_slice(&(control.len() as u64 + 100).to_le_bytes()); // Claim it's longer
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&control); // Write real data (shorter)

    let mut rom = vec![0; 10];
    let patcher = BdfPatcher;
    // Should fail when reading diff block start because control block claimed to be
    // longer
    assert!(patcher.apply(&mut rom, &patch).is_err());
}

#[test]
fn test_apply_invalid_bzip_header() {
    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    let control = vec![0x00, 0x01, 0x02]; // Not valid bzip
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());
    patch.extend_from_slice(&control);

    let mut rom = vec![0; 10];
    let patcher = BdfPatcher;
    // Should fail inside bzip decoder
    assert!(patcher.apply(&mut rom, &patch).is_err());
}

#[test]
fn test_apply_truncated_diff_block() {
    // Setup a valid control block that says we have 1 diff byte
    let mut control_data = Vec::new();
    control_data.extend_from_slice(&1u64.to_le_bytes()); // diff_len = 1
    control_data.extend_from_slice(&0u64.to_le_bytes()); // extra_len = 0
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek_len = 0
    let control = create_valid_bzip_block(&control_data);

    // But provide empty diff block
    let diff = create_valid_bzip_block(&[]);

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes()); // valid patched size

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&[]); // empty extra

    let mut rom = vec![0; 10];
    let patcher = BdfPatcher;
    let result = patcher.apply(&mut rom, &patch);
    assert!(result.is_err());
}

#[test]
fn test_apply_truncated_extra_block() {
    // Setup a valid control block: 0 diff, 5 extra
    let mut control_data = Vec::new();
    control_data.extend_from_slice(&0u64.to_le_bytes()); // diff_len = 0
    control_data.extend_from_slice(&5u64.to_le_bytes()); // extra_len = 5
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek_len = 0
    let control = create_valid_bzip_block(&control_data);

    let diff = create_valid_bzip_block(&[]);
    let extra = create_valid_bzip_block(&[1, 2]); // Only 2 bytes, need 5

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&extra);

    let mut rom = vec![0; 10];
    let patcher = BdfPatcher;
    assert!(patcher.apply(&mut rom, &patch).is_err());
}

#[test]
fn test_apply_malformed_control_entry() {
    // Control block that ends abruptly in the middle of a u64
    let control_data = vec![0x00, 0x01, 0x02]; // 3 bytes, need 24 for full entry
    let control = create_valid_bzip_block(&control_data);
    let diff = create_valid_bzip_block(&[]);
    let extra = create_valid_bzip_block(&[]);

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff.len() as u64).to_le_bytes());
    patch.extend_from_slice(&0u64.to_le_bytes());

    patch.extend_from_slice(&control);
    patch.extend_from_slice(&diff);
    patch.extend_from_slice(&extra);

    let mut rom = vec![0; 10];
    let patcher = BdfPatcher;
    let result = patcher.apply(&mut rom, &patch);

    // Might be CorruptedData or IO error from bzip reader
    assert!(result.is_err());
}
