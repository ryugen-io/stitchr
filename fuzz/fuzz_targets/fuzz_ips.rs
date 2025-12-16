#![no_main]
use libfuzzer_sys::fuzz_target;
use stitchr_formats::ips::IpsPatcher;
use stitchr_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // Only fuzz if it looks somewhat like IPS or if we want to force it.
    // Ideally we fuzz everything, but let's check can_handle to avoid noise?
    // Actually, fuzzing validate() on ANY data is good to find parsing crashes.
    // But apply() usually expects valid headers.
    
    // 1. Fuzz validation (should handle garbage safely)
    let _ = IpsPatcher::validate(data);
    
    // 2. Fuzz metadata (should handle garbage safely)
    let _ = IpsPatcher::metadata(data);

    // 3. Fuzz application
    // We create a small dummy ROM. IPS can write anywhere, so it might extend the vector.
    // To treat it realistically, we only try apply if it passes basic validation?
    // No, apply() should also be robust against garbage.
    let mut rom = vec![0u8; 256]; // Small ROM
    let patcher = IpsPatcher;
    let _ = patcher.apply(&mut rom, data);
});
