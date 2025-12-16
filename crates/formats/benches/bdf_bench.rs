use bzip2::Compression;
use bzip2::write::BzEncoder;
use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::bdf::{BdfPatcher, constants::BDF_MAGIC};
use std::io::Write;

fn main() {
    divan::main();
}

fn create_bdf_patch(_old_data: &[u8], new_data: &[u8]) -> Vec<u8> {
    let mut control_data = Vec::new();
    control_data.extend_from_slice(&0u64.to_le_bytes()); // diff_len
    control_data.extend_from_slice(&(new_data.len() as u64).to_le_bytes()); // extra_len
    control_data.extend_from_slice(&0i64.to_le_bytes()); // seek_len

    let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&control_data).unwrap();
    let control_compressed = encoder.finish().unwrap();

    let encoder = BzEncoder::new(Vec::new(), Compression::default());
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

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
];

#[divan::bench(args = SIZES)]
fn bdf_apply(bencher: Bencher, size: usize) {
    let old = vec![0u8; size];
    let new = vec![0xFFu8; size];
    let patch = create_bdf_patch(&old, &new);

    bencher.bench_local(|| {
        let mut rom = old.clone();
        BdfPatcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn bdf_validate(bencher: Bencher, size: usize) {
    let old = vec![0u8; size];
    let new = vec![0xFFu8; size];
    let patch = create_bdf_patch(&old, &new);

    bencher.bench(|| {
        BdfPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}
