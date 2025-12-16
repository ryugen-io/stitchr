use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::aps::gba::ApsGbaPatcher;
use stitchr_formats::aps::n64::ApsN64Patcher;

fn main() {
    divan::main();
}

fn generate_test_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS10");
    patch.push(0x01);
    patch.push(0x00);
    patch.extend_from_slice(&[0u8; 50]);
    patch.push(0x01);
    patch.extend_from_slice(b"TST");
    patch.extend_from_slice(&[0u8; 8]);
    patch.extend_from_slice(&[0u8; 5]);
    patch.extend_from_slice(&(rom_size as u32).to_le_bytes());

    let interval = rom_size / patch_count.max(1);
    for i in 0..patch_count {
        let offset = (i * interval) as u32;
        patch.extend_from_slice(&offset.to_le_bytes());
        patch.push(1);
        patch.push(0xFF);
    }

    patch
}

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
    4 * 1024 * 1024,  // 4MB
    8 * 1024 * 1024,  // 8MB
    16 * 1024 * 1024, // 16MB
    32 * 1024 * 1024, // 32MB
];

#[divan::bench(args = SIZES)]
fn aps_apply(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        ApsN64Patcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn aps_validate(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        ApsN64Patcher::validate(divan::black_box(&patch)).unwrap();
    });
}

fn generate_gba_patch(rom_size: usize) -> Vec<u8> {
    const BLOCK_SIZE: usize = 0x10000;
    let mut patch = Vec::new();
    patch.extend_from_slice(b"APS1");
    patch.extend_from_slice(&(rom_size as u32).to_le_bytes());
    patch.extend_from_slice(&(rom_size as u32).to_le_bytes());
    patch.extend_from_slice(&0u32.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&0u16.to_le_bytes());
    patch.extend_from_slice(&vec![0xFFu8; BLOCK_SIZE]);
    patch
}

#[divan::bench(args = SIZES)]
fn aps_gba_apply(bencher: Bencher, size: usize) {
    let patch = generate_gba_patch(size);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        ApsGbaPatcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn aps_gba_validate(bencher: Bencher, size: usize) {
    let patch = generate_gba_patch(size);

    bencher.bench(|| {
        ApsGbaPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}
