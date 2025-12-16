use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::ppf::PpfPatcher;

fn main() {
    divan::main();
}

fn generate_ppf_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PPF30"); // Magic
    patch.push(0x02); // Encoding
    patch.extend_from_slice(&[0u8; 50]); // Description
    patch.push(0x00); // Image type
    patch.push(0x00); // Block check
    patch.push(0x00); // Undo data
    patch.push(0x00); // Dummy

    let interval = rom_size / patch_count.max(1);
    for i in 0..patch_count {
        let offset = (i * interval) as u64;
        
        // Offset (8 bytes LE)
        patch.extend_from_slice(&offset.to_le_bytes());
        // Length (1 byte) - writing 1 byte
        patch.push(0x01);
        // Data (1 byte)
        patch.push(0xFF);
    }
    patch
}

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
    16 * 1024 * 1024, // 16MB
];

#[divan::bench(args = SIZES)]
fn ppf_apply(bencher: Bencher, size: usize) {
    let patch = generate_ppf_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        PpfPatcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn ppf_validate(bencher: Bencher, size: usize) {
    let patch = generate_ppf_patch(size, 10);

    bencher.bench(|| {
        PpfPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}
