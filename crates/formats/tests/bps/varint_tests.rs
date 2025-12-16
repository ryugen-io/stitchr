//! BPS varint decoding tests

use stitchr_formats::bps::varint;

#[test]
fn test_decode_single_byte() {
    // 0x80 = 0b10000000 -> value 0, termination bit set
    assert_eq!(varint::decode(&[0x80]).unwrap(), (0, 1));

    // 0x81 = 0b10000001 -> value 1
    assert_eq!(varint::decode(&[0x81]).unwrap(), (1, 1));

    // 0xFF = 0b11111111 -> value 127
    assert_eq!(varint::decode(&[0xFF]).unwrap(), (127, 1));
}

#[test]
fn test_decode_multi_byte() {
    // 0x00 0x80 -> 128
    assert_eq!(varint::decode(&[0x00, 0x80]).unwrap(), (128, 2));

    // 0x01 0x80 -> 129
    assert_eq!(varint::decode(&[0x01, 0x80]).unwrap(), (129, 2));
}

#[test]
fn test_decode_overflow() {
    // Too many bytes without termination
    let long_varint = vec![0x00; 11];
    assert!(varint::decode(&long_varint).is_err());
}

#[test]
fn test_decode_incomplete() {
    // No termination bit
    assert!(varint::decode(&[0x00]).is_err());
    assert!(varint::decode(&[0x7F]).is_err());
}
