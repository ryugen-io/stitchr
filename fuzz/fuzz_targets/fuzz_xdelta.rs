#![no_main]
use libfuzzer_sys::fuzz_target;
use rom_patcher_formats::xdelta::XdeltaPatcher;
use rom_patcher_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz validation
    let _ = XdeltaPatcher::validate(data);
    
    // 2. Fuzz metadata                                                              
    let _ = XdeltaPatcher::metadata(data);

    // 3. Fuzz application
    let mut rom = vec![0u8; 256];
    let patcher = XdeltaPatcher;
    let _ = patcher.apply(&mut rom, data);
});
