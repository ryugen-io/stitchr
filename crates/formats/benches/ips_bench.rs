use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::ips::IpsPatcher;

fn main() {
    divan::main();
}

/// Generate a test IPS patch that writes 0xFF at 10 evenly-spaced offsets
fn generate_test_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH"); // Header

    let interval = rom_size / patch_count.max(1);
    for i in 0..patch_count {
        let offset = (i * interval) as u32;
        // Write offset (3 bytes BE)
        patch.push((offset >> 16) as u8);
        patch.push((offset >> 8) as u8);
        patch.push(offset as u8);
        // Write size (2 bytes BE)
        patch.push(0x00);
        patch.push(0x01);
        // Write data (1 byte)
        patch.push(0xFF);
    }

    patch.extend_from_slice(b"EOF"); // Footer
    patch
}

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
    4 * 1024 * 1024,  // 4MB
    8 * 1024 * 1024,  // 8MB
    16 * 1024 * 1024, // 16MB (IPS max)
];

#[divan::bench(args = SIZES)]
fn ips_apply(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        IpsPatcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn ips_validate(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        IpsPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}
