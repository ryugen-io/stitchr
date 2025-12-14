use bzip2::Compression;
use bzip2::write::BzEncoder;
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};
use std::io::Write;

fn create_bdf_patch(_old_data: &[u8], new_data: &[u8]) -> Vec<u8> {
    // ...
    let mut control_data = Vec::new();
    control_data.extend_from_slice(&0u64.to_le_bytes()); // diff_len
    control_data.extend_from_slice(&(new_data.len() as u64).to_le_bytes()); // extra_len
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek_len

    let mut encoder = BzEncoder::new(Vec::new(), Compression::default()); // This one is mutated by write_all below?
    encoder.write_all(&control_data).unwrap();
    let control_compressed = encoder.finish().unwrap();

    let encoder = BzEncoder::new(Vec::new(), Compression::default()); // This one is not written to?
    // Empty diff block
    let diff_compressed = encoder.finish().unwrap();

    let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(new_data).unwrap();
    let extra_compressed = encoder.finish().unwrap();

    let mut patch = Vec::new();
    patch.extend_from_slice(BDF_MAGIC);
    patch.extend_from_slice(&(control_compressed.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(diff_compressed.len() as u64).to_le_bytes());
    patch.extend_from_slice(&(new_data.len() as u64).to_le_bytes());

    patch.extend_from_slice(&control_compressed);
    patch.extend_from_slice(&diff_compressed);
    patch.extend_from_slice(&extra_compressed);

    patch
}

#[test]
fn test_apply_replacement() {
    let old_rom = vec![0xAA; 10];
    let new_rom_data = vec![0xBB; 10];
    let patch = create_bdf_patch(&old_rom, &new_rom_data);

    let mut rom = old_rom.clone();
    let patcher = BdfPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom, new_rom_data);
}

#[test]
fn test_apply_resize() {
    let old_rom = vec![0xAA; 10];
    let new_rom_data = vec![0xBB; 20]; // Larger
    let patch = create_bdf_patch(&old_rom, &new_rom_data);

    let mut rom = old_rom.clone();
    let patcher = BdfPatcher;
    patcher.apply(&mut rom, &patch).unwrap();

    assert_eq!(rom.len(), 20);
    assert_eq!(rom, new_rom_data);
}
