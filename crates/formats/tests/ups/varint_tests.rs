//! UPS varint (VLV) decoding tests

use rom_patcher_formats::ups::varint;

#[test]
fn test_varint_decode() {
    // Test decoding of known valid values
    assert_eq!(varint::decode(&[0x80]).unwrap(), (0, 1));
    assert_eq!(varint::decode(&[0x81]).unwrap(), (1, 1));
    assert_eq!(varint::decode(&[0xFF]).unwrap(), (127, 1));
    assert_eq!(varint::decode(&[0x00, 0x80]).unwrap(), (128, 2));
}
