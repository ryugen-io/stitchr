use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::ups::UpsPatcher;

fn main() {
    divan::main();
}

/// Generate a test UPS patch with XOR records
fn generate_test_patch(rom_size: usize, xor_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();

    // Magic header
    patch.extend_from_slice(b"UPS1");

    // Input size (VLV)
    write_vlv(&mut patch, rom_size as u64);

    // Output size (VLV) - same as source for simplicity
    write_vlv(&mut patch, rom_size as u64);

    // Generate XOR records
    let stride = if xor_count > 0 {
        rom_size / xor_count
    } else {
        rom_size
    };
    for i in 0..xor_count {
        let relative_offset = if i == 0 { 0 } else { stride - 5 }; // -5 for XOR data length
        write_vlv(&mut patch, relative_offset as u64);

        // XOR data (4 bytes)
        patch.push(0xFF);
        patch.push(0xEE);
        patch.push(0xDD);
        patch.push(0xCC);

        // Terminator
        patch.push(0x00);
    }

    // Checksums (dummy values for benchmark)
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // input CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // output CRC32

    // Patch CRC32 (compute real value)
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    patch
}

fn write_vlv(buf: &mut Vec<u8>, mut data: u64) {
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

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
    4 * 1024 * 1024,  // 4MB
    8 * 1024 * 1024,  // 8MB
    16 * 1024 * 1024, // 16MB
    32 * 1024 * 1024, // 32MB (like Mother 3)
];

#[divan::bench(args = SIZES)]
fn ups_apply(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        // Note: Will fail CRC validation but exercises the apply logic
        let _ = UpsPatcher.apply(divan::black_box(&mut rom), divan::black_box(&patch));
    });
}

#[divan::bench(args = SIZES)]
fn ups_validate(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        UpsPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn ups_metadata(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        UpsPatcher::metadata(divan::black_box(&patch)).unwrap();
    });
}
