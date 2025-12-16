//! xdelta header parsing tests

use stitchr_formats::xdelta::{headers::WindowHeader, parser::VcdiffParser};

#[test]
fn test_decode_window_header_minimal() {
    // Indicator: 0x00 (No Source/Target, No Adler)
    // Delta Length: 0
    // Target Window Length: 0
    // Delta Indicator: 0
    // Add Run Data Length: 0
    // Inst Length: 0
    // Addr Length: 0
    let data = vec![
        0x00, // Indicator
        0x00, // Delta Length (7-bit)
        0x00, // Target Length
        0x00, // Delta Indicator
        0x00, // Add Run
        0x00, // Inst
        0x00, // Addr
    ];

    let mut parser = VcdiffParser::new(&data);
    let header = WindowHeader::decode(&mut parser).unwrap();

    assert_eq!(header.indicator, 0);
    assert_eq!(header.target_window_length, 0);
}
