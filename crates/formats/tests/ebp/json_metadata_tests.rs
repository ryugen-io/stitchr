//! EBP-specific JSON metadata tests

use stitchr_core::PatchFormat;
use stitchr_formats::ebp::EbpPatcher;

#[test]
fn test_json_title() {
    let patch = b"PATCHEOF{\"title\":\"Test\"}";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    let title = metadata.extra.iter().find(|(k, _)| k == "title");
    assert_eq!(title.unwrap().1, "Test");
}

#[test]
fn test_json_author() {
    let patch = b"PATCHEOF{\"author\":\"Me\"}";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    let author = metadata.extra.iter().find(|(k, _)| k == "author");
    assert_eq!(author.unwrap().1, "Me");
}

#[test]
fn test_json_full() {
    let patch =
        b"PATCHEOF{\"title\":\"T\",\"author\":\"A\",\"description\":\"D\",\"version\":\"1\"}";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    assert_eq!(metadata.extra.len(), 4);
}

#[test]
fn test_json_escaped() {
    let patch = b"PATCHEOF{\"title\":\"L1\\nL2\"}";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    let title = metadata.extra.iter().find(|(k, _)| k == "title");
    assert_eq!(title.unwrap().1, "L1\nL2");
}

#[test]
fn test_json_invalid() {
    let patch = b"PATCHEOF{invalid}";
    let metadata = EbpPatcher::metadata(patch).unwrap();
    assert!(metadata.extra.is_empty());
}

#[test]
fn test_apply_with_json() {
    let mut rom = vec![0x00; 10];
    let patch = b"PATCH\x00\x00\x05\x00\x01\xAAEOF{\"title\":\"Test\"}";

    let patcher = EbpPatcher;
    patcher.apply(&mut rom, patch).unwrap();
    assert_eq!(rom[5], 0xAA);
}
