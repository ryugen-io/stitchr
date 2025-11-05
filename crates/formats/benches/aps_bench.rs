use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::aps::gba::ApsGbaPatcher;
use rom_patcher_formats::aps::n64::ApsN64Patcher;

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

fn bench_aps_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("aps_apply");
    group.measurement_time(std::time::Duration::from_secs(15));

    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB
        32 * 1024 * 1024, // 32MB
    ]
    .iter()
    {
        let patch = generate_test_patch(*size, 10);
        let original = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut rom = original.clone();
                ApsN64Patcher
                    .apply(black_box(&mut rom), black_box(&patch))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_aps_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("aps_validate");

    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB
        32 * 1024 * 1024, // 32MB
    ]
    .iter()
    {
        let patch = generate_test_patch(*size, 10);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                ApsN64Patcher::validate(black_box(&patch)).unwrap();
            });
        });
    }

    group.finish();
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

fn bench_aps_gba_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("aps_gba_apply");
    group.measurement_time(std::time::Duration::from_secs(15));

    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB
        32 * 1024 * 1024, // 32MB
    ]
    .iter()
    {
        let patch = generate_gba_patch(*size);
        let original = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut rom = original.clone();
                ApsGbaPatcher
                    .apply(black_box(&mut rom), black_box(&patch))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_aps_gba_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("aps_gba_validate");

    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB
        32 * 1024 * 1024, // 32MB
    ]
    .iter()
    {
        let patch = generate_gba_patch(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                ApsGbaPatcher::validate(black_box(&patch)).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_aps_apply,
    bench_aps_validate,
    bench_aps_gba_apply,
    bench_aps_gba_validate
);
criterion_main!(benches);
