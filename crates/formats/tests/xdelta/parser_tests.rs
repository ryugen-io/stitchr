//! xdelta parser unit tests

use stitchr_formats::xdelta::parser::VcdiffParser;

#[test]
fn test_read_7bit_int_single_byte() {
    let data = [0x05];
    let mut parser = VcdiffParser::new(&data);
    assert_eq!(parser.read_7bit_encoded_int().unwrap(), 5);
}

#[test]
fn test_read_7bit_int_multi_byte() {
    // 129 = 1 * 128 + 1 -> encoded as [0x81, 0x01] (High bit set on first)
    // 0x81 = 1000 0001. Val = 1.
    // 0x01 = 0000 0001. Val = 1.
    // Result = (1 << 7) + 1 = 129.
    let data = [0x81, 0x01];
    let mut parser = VcdiffParser::new(&data);
    assert_eq!(parser.read_7bit_encoded_int().unwrap(), 129);
}

#[test]
fn test_read_7bit_int_max_value() {
    // 64-bit max is tricky to encode manually, but let's try a 3-byte one.
    // [0xFF, 0xFF, 0x7F] -> 127 << 14 + 127 << 7 + 127
    // = 2080768 + 16256 + 127 = 2097151
    let data = [0xFF, 0xFF, 0x7F];
    let mut parser = VcdiffParser::new(&data);
    assert_eq!(parser.read_7bit_encoded_int().unwrap(), 2097151);
}
