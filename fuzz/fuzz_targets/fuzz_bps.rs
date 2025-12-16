#![no_main]
use libfuzzer_sys::fuzz_target;
use stitchr_formats::bps::BpsPatcher;
use stitchr_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    let _ = BpsPatcher::validate(data);
    let _ = BpsPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = BpsPatcher;
    let _ = patcher.apply(&mut rom, data);
});
