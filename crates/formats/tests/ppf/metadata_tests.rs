//! PPF metadata extraction tests.

use stitchr_core::PatchFormat;
use stitchr_formats::ppf::PpfPatcher;

#[test]
fn test_metadata_simple_ppf3() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30");
    patch_data.push(0x02); // Encoding

    // Description: "Test Patch" + padding
    let mut desc = vec![0u8; 50];
    let desc_str = b"Test Patch";
    desc[..desc_str.len()].copy_from_slice(desc_str);
    patch_data.extend_from_slice(&desc);

    patch_data.push(0x00); // Image Type
    patch_data.push(0x01); // Block Check (True)
    patch_data.push(0x01); // Undo Data (True)
    patch_data.push(0x00); // Dummy

    // Block Check Data (1024 bytes)
    patch_data.extend_from_slice(&[0u8; 1024]);

    // Record: Offset 0, Len 1, Data 0xAA, Undo 0xBB
    patch_data.extend_from_slice(&0u64.to_le_bytes());
    patch_data.push(0x01);
    patch_data.push(0xAA);
    patch_data.push(0xBB); // Undo data

    // metadata is a static method in the trait
    let metadata = PpfPatcher::metadata(&patch_data).unwrap();

    let get_extra = |key: &str| -> Option<&str> {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v): &(String, String)| v.as_str())
    };

    assert_eq!(get_extra("description"), Some("Test Patch"));
    assert_eq!(get_extra("version"), Some("PPF3"));
    assert_eq!(get_extra("encoding_method"), Some("2"));
    assert_eq!(get_extra("block_check"), Some("true"));
    assert_eq!(get_extra("undo_data"), Some("true"));
}

#[test]
fn test_metadata_ppf2_fields() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF20");
    patch_data.push(0x01);
    patch_data.extend_from_slice(&[0u8; 50]);
    patch_data.extend_from_slice(&12345u32.to_le_bytes()); // Input size
    patch_data.extend_from_slice(&[0u8; 1024]); // Block check

    let metadata = PpfPatcher::metadata(&patch_data).unwrap();
    let get_extra = |key: &str| {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v): &(String, String)| v.as_str())
    };

    assert_eq!(get_extra("version"), Some("PPF2"));
    assert_eq!(get_extra("input_file_size"), Some("12345"));
    assert_eq!(get_extra("block_check"), Some("true"));
}

#[test]
fn test_metadata_file_id_diz_extraction() {
    let mut patch_data = Vec::new();
    patch_data.extend_from_slice(b"PPF30");
    patch_data.push(0x02);
    patch_data.extend_from_slice(&[0u8; 50]);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);
    patch_data.push(0x00);

    // DIZ block
    patch_data.extend_from_slice(b"@BEG");
    patch_data.extend_from_slice(b"Test DIZ Content");
    patch_data.extend_from_slice(b"@END_FILE_ID.DIZ");

    let metadata = PpfPatcher::metadata(&patch_data).unwrap();
    let get_extra = |key: &str| {
        metadata
            .extra
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v): &(String, String)| v.as_str())
    };

    assert_eq!(get_extra("file_id_diz"), Some("Test DIZ Content"));
}
