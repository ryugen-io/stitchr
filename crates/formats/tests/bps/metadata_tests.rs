//! Tests for BPS metadata extraction

use rom_patcher_core::PatchFormat;
use rom_patcher_formats::bps::BpsPatcher;

fn write_varint(buf: &mut Vec<u8>, mut data: u64) {
    loop {
        let x = (data & 0x7f) as u8;
        data >>= 7;
        if data == 0 {
            buf.push(0x80 | x);
            break;
        }
        buf.push(x);
        data -= 1;
    }
}

#[test]
fn test_metadata_simple() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    write_varint(&mut patch, 1024); // source_size
    write_varint(&mut patch, 2048); // target_size
    write_varint(&mut patch, 0); // metadata_size
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let metadata = BpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.source_size, Some(1024));
    assert_eq!(metadata.target_size, Some(2048));
}

#[test]
fn test_metadata_with_info() {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"BPS1");
    write_varint(&mut patch, 512); // source_size
    write_varint(&mut patch, 1024); // target_size
    let metadata_str = b"Test patch v1.0";
    write_varint(&mut patch, metadata_str.len() as u64); // metadata_size
    patch.extend_from_slice(metadata_str);
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // source CRC32
    patch.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // target CRC32
    let patch_crc = crc32fast::hash(&patch);
    patch.extend_from_slice(&patch_crc.to_le_bytes());

    let metadata = BpsPatcher::metadata(&patch).unwrap();

    assert_eq!(metadata.source_size, Some(512));
    assert_eq!(metadata.target_size, Some(1024));
}

#[test]
fn test_metadata_invalid_patch() {
    let invalid_patch = b"NOTBPS";
    let result = BpsPatcher::metadata(invalid_patch);

    assert!(result.is_err());
}
