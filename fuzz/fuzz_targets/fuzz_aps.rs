#![no_main]
use libfuzzer_sys::fuzz_target;
use stitchr_formats::aps::ApsPatcher;
use stitchr_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // APS checks strict signatures in keys, so many random inputs fail fast.
    let _ = ApsPatcher::validate(data);
    let _ = ApsPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = ApsPatcher;
    let _ = patcher.apply(&mut rom, data);
});
