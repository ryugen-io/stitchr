//! Tests for RUP VLV (Variable-Length Value) encoding

use stitchr_formats::rup::varint::decode_vlv;

#[test]
fn test_decode_zero() {
    let data = [0x00];
    let (value, consumed) = decode_vlv(&data).unwrap();
    assert_eq!(value, 0);
    assert_eq!(consumed, 1);
}

#[test]
fn test_decode_single_byte() {
    let data = [0x01, 0x42];
    let (value, consumed) = decode_vlv(&data).unwrap();
    assert_eq!(value, 0x42);
    assert_eq!(consumed, 2);
}

#[test]
fn test_decode_two_bytes() {
    let data = [0x02, 0x34, 0x12]; // 0x1234 little-endian
    let (value, consumed) = decode_vlv(&data).unwrap();
    assert_eq!(value, 0x1234);
    assert_eq!(consumed, 3);
}

#[test]
fn test_decode_four_bytes() {
    let data = [0x04, 0x78, 0x56, 0x34, 0x12]; // 0x12345678 LE
    let (value, consumed) = decode_vlv(&data).unwrap();
    assert_eq!(value, 0x12345678);
    assert_eq!(consumed, 5);
}

#[test]
fn test_decode_eight_bytes() {
    let data = [0x08, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
    let (value, consumed) = decode_vlv(&data).unwrap();
    assert_eq!(value, 0x1122334455667788);
    assert_eq!(consumed, 9);
}

#[test]
fn test_decode_empty_data() {
    let data = [];
    assert!(decode_vlv(&data).is_err());
}

#[test]
fn test_decode_truncated() {
    let data = [0x04, 0x12, 0x34]; // Says 4 bytes but only has 2
    assert!(decode_vlv(&data).is_err());
}
