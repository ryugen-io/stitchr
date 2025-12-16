use divan::Bencher;
use stitchr_core::PatchFormat;
use stitchr_formats::xdelta::XdeltaPatcher;

fn main() {
    divan::main();
}

// We use the existing test file for benchmarking
const PATCH_BYTES: &[u8] = include_bytes!("../../../test_files/xdelta/patch.xdelta");
const ROM_BYTES: &[u8] = include_bytes!("../../../test_files/xdelta/test.rom.nds");

#[divan::bench]
fn xdelta_validate(bencher: Bencher) {
    bencher.bench(|| {
        XdeltaPatcher::validate(divan::black_box(PATCH_BYTES)).unwrap();
    });
}

// Now with actual ROM bytes
#[divan::bench]
fn xdelta_apply(bencher: Bencher) {
    bencher.bench_local(|| {
        let mut rom = ROM_BYTES.to_vec(); // Clone to make it mutable
        // Ignore result as we expect failure or partial success depending on patch content
        let _ = XdeltaPatcher.apply(divan::black_box(&mut rom), divan::black_box(PATCH_BYTES));
    });
}
