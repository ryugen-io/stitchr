use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::rup::RupPatcher;

fn main() {
    divan::main();
}

fn write_vlv(buf: &mut Vec<u8>, value: u64) {
    if value == 0 {
        buf.push(0);
        return;
    }
    let bytes_needed = ((64 - value.leading_zeros()).div_ceil(8)) as u8;
    buf.push(bytes_needed);
    for i in 0..bytes_needed {
        buf.push((value >> (i * 8)) as u8);
    }
}

fn generate_test_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let original = vec![0u8; rom_size];
    let source_md5 = md5::compute(&original);

    let mut target = original.clone();
    let interval = rom_size / patch_count.max(1);
    for i in 0..patch_count {
        let offset = i * interval;
        if offset < target.len() {
            target[offset] ^= 0xFF;
        }
    }
    let target_md5 = md5::compute(&target);

    let mut patch = vec![0u8; 0x800];
    patch[0..6].copy_from_slice(b"NINJA2");
    patch.push(0x01);
    write_vlv(&mut patch, 8);
    patch.extend_from_slice(b"test.rom");
    patch.push(3);
    write_vlv(&mut patch, rom_size as u64);
    write_vlv(&mut patch, rom_size as u64);
    patch.extend_from_slice(&source_md5.0);
    patch.extend_from_slice(&target_md5.0);

    for i in 0..patch_count {
        patch.push(0x02);
        write_vlv(&mut patch, (i * interval) as u64);
        write_vlv(&mut patch, 1);
        patch.push(0xFF);
    }
    patch.push(0x00);
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
fn rup_apply(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);
    let original = vec![0u8; size];

    bencher.bench_local(|| {
        let mut rom = original.clone();
        RupPatcher
            .apply(divan::black_box(&mut rom), divan::black_box(&patch))
            .unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn rup_validate(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        RupPatcher::validate(divan::black_box(&patch)).unwrap();
    });
}

#[divan::bench(args = SIZES)]
fn rup_metadata(bencher: Bencher, size: usize) {
    let patch = generate_test_patch(size, 10);

    bencher.bench(|| {
        RupPatcher::metadata(divan::black_box(&patch)).unwrap();
    });
}
