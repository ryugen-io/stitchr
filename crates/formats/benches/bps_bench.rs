use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::bps::BpsPatcher;

fn main() {
    divan::main();
}

/// Generate a test BPS patch with mixed actions
fn generate_test_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();

    // Magic header
    patch.extend_from_slice(b"BPS1");

    // Source size (varint)
    write_varint(&mut patch, rom_size as u64);

    // Target size (varint) - same as source for simplicity
    write_varint(&mut patch, rom_size as u64);

    // Metadata size (0)
    write_varint(&mut patch, 0);

    // Generate actions: mix of SOURCE_READ and TARGET_READ
    for i in 0..patch_count {
        let action_type = i % 2; // Alternate between SOURCE_READ(0) and TARGET_READ(1)
        let length = 10; // Fixed length for simplicity

        // Action encoding: ((length-1)<<2) | type
        let action = ((length - 1) << 2) | action_type;
        write_varint(&mut patch, action as u64);

        // TARGET_READ needs data
        if action_type == 1 {
            patch.extend(std::iter::repeat_n(0xFF, length));
        }
    }

    // Checksums (dummy values for benchmark)
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

const SIZES: &[usize] = &[
    1024,             // 1KB
    10 * 1024,        // 10KB
    100 * 1024,       // 100KB
    1024 * 1024,      // 1MB
    4 * 1024 * 1024,  // 4MB
    8 * 1024 * 1024,  // 8MB
    16 * 1024 * 1024, // 16MB (IPS max, but BPS can go beyond)
];

#[divan::bench(args = SIZES)]
fn bps_apply(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        // Note: Will fail CRC validation but exercises the apply logic
        let _ = BpsPatcher.apply(divan::black_box(&mut rom), divan::black_box(&patch));
    });
}

#[divan::bench(args = SIZES)]
fn bps_validate(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        BpsPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn bps_metadata(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        BpsPatcher::metadata(divan::black_box(&patch)).unwrap();
    });
}
