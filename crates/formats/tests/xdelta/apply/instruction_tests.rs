//! xdelta instruction tests

use crate::xdelta::helpers::{VcdiffWindowBuilder, prepend_header};
use stitchr_core::PatchFormat;
use stitchr_formats::xdelta::XdeltaPatcher;

#[test]
fn test_inst_add_embedded_size() {
    // Code 2 = ADD size 1.
    // Add Data: 0x42.
    let inst = [0x02];
    let add = [0x42];

    let window = VcdiffWindowBuilder::new(1)
        .with_instructions(&inst)
        .with_add(&add)
        .build();
    let patch = prepend_header(&window);

    let mut rom = vec![];
    XdeltaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom, vec![0x42]);
}

#[test]
fn test_inst_add_separate_size() {
    // Code 1 = ADD size 0 (read next).
    // Inst: 0x01, Size: 0x02.
    // Add Data: 0x42, 0x43.
    let inst = [0x01, 0x02];
    let add = [0x42, 0x43];

    let window = VcdiffWindowBuilder::new(2)
        .with_instructions(&inst)
        .with_add(&add)
        .build();
    let patch = prepend_header(&window);

    let mut rom = vec![];
    XdeltaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom, vec![0x42, 0x43]);
}

#[test]
fn test_inst_run() {
    // Code 0 = RUN size 0 (read next).
    // Inst: 0x00, Size: 0x03.
    // Add Data: 0xAA (Run byte).
    let inst = [0x00, 0x03];
    let add = [0xAA];

    let window = VcdiffWindowBuilder::new(3)
        .with_instructions(&inst)
        .with_add(&add)
        .build();
    let patch = prepend_header(&window);

    let mut rom = vec![];
    XdeltaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom, vec![0xAA, 0xAA, 0xAA]);
}
