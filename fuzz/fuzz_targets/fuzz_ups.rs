#![no_main]
use libfuzzer_sys::fuzz_target;
use stitchr_formats::ups::UpsPatcher;
use stitchr_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    let _ = UpsPatcher::validate(data);
    let _ = UpsPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = UpsPatcher;
    let _ = patcher.apply(&mut rom, data);
});
