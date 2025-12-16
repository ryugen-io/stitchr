//! xdelta window tests

use crate::xdelta::helpers::{VcdiffWindowBuilder, prepend_header};
use stitchr_core::{PatchError, PatchFormat};
use stitchr_formats::xdelta::XdeltaPatcher;

#[test]
fn test_apply_multi_window_concatenation() {
    // Window 1: ADD "Hello "
    // ADD size 6 = Code 7.
    let w1 = VcdiffWindowBuilder::new(6)
        .with_instructions(&[0x07])
        .with_add(b"Hello ")
        .build();

    // Window 2: ADD "World"
    // ADD size 5 = Code 6.
    let w2 = VcdiffWindowBuilder::new(5)
        .with_instructions(&[0x06])
        .with_add(b"World")
        .build();

    // Combine: Header + w1 + w2
    let mut patch = Vec::new();
    patch.extend_from_slice(&[0xD6, 0xC3, 0xC4, 0x00]); // Magic
    patch.push(0x00); // Hdr Indicator
    patch.extend_from_slice(&w1);
    patch.extend_from_slice(&w2);

    let mut rom = vec![];
    XdeltaPatcher.apply(&mut rom, &patch).unwrap();
    assert_eq!(rom, b"Hello World");
}

#[test]
fn test_window_checksum_failure() {
    let mut builder = VcdiffWindowBuilder::new(1);
    builder.adler32 = Some(0xDEADBEEF); // Wrong checksum
    let win = builder
        .with_instructions(&[0x02]) // ADD size 1
        .with_add(b"A")
        .build();

    let patch = prepend_header(&win);

    let mut rom = vec![];
    let result = XdeltaPatcher.apply(&mut rom, &patch);
    assert!(matches!(result, Err(PatchError::ChecksumMismatch { .. })));
}
