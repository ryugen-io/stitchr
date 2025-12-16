#![no_main]
use libfuzzer_sys::fuzz_target;
use stitchr_formats::ppf::PpfPatcher;
use stitchr_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz validation (should handle garbage safely)
    let _ = PpfPatcher::validate(data);
    
    // 2. Fuzz metadata (should handle garbage safely)
    let _ = PpfPatcher::metadata(data);

    // 3. Fuzz application
    // PPF patches usually apply to large ISOs, but for fuzzing we use a small buffer.
    // The patcher checks bounds, so it should just return an error, not panic.
    let mut rom = vec![0u8; 1024]; // Small ROM
    let patcher = PpfPatcher;
    let _ = patcher.apply(&mut rom, data);
});
