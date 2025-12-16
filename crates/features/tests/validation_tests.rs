//! Validation feature tests

use stitchr_features::validation::{HashAlgorithm, ValidationFeature, Validator};

#[cfg(feature = "validation")]
use stitchr_features::validation::algorithms::md5;

#[test]
fn test_crc32_empty() {
    let data = b"";
    let validator = Validator::new();
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_crc32_known_values() {
    let validator = Validator::new();

    // Test "123456789"
    let data = b"123456789";
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0xCB, 0xF4, 0x39, 0x26]);

    // Test "The quick brown fox jumps over the lazy dog"
    let data = b"The quick brown fox jumps over the lazy dog";
    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert_eq!(hash, vec![0x41, 0x4F, 0xA3, 0x39]);
}

#[test]
fn test_validation() {
    let data = b"test data";
    let validator = Validator::new();

    let hash = validator.compute_hash(data, HashAlgorithm::Crc32);
    assert!(
        validator
            .validate_checksum(data, &hash, HashAlgorithm::Crc32)
            .is_ok()
    );

    let wrong_hash = vec![0x00, 0x00, 0x00, 0x00];
    assert!(
        validator
            .validate_checksum(data, &wrong_hash, HashAlgorithm::Crc32)
            .is_err()
    );
}

#[test]
#[cfg(feature = "validation")]
fn test_md5_empty() {
    let data = b"";
    let hash = md5::compute(data);
    assert_eq!(hash, "d41d8cd98f00b204e9800998ecf8427e");
}

#[test]
#[cfg(feature = "validation")]
fn test_md5_known_values() {
    // Test "The quick brown fox jumps over the lazy dog"
    let data = b"The quick brown fox jumps over the lazy dog";
    let hash = md5::compute(data);
    assert_eq!(hash, "9e107d9d372bb6826bd81d3542a419d6");

    // Test "123456789"
    let data = b"123456789";
    let hash = md5::compute(data);
    assert_eq!(hash, "25f9e794323b453885f5181f1b624d0b");

    // Test single byte
    let data = b"a";
    let hash = md5::compute(data);
    assert_eq!(hash, "0cc175b9c0f1b6a831c399e269772661");
}

#[test]
#[cfg(feature = "validation")]
fn test_md5_binary_data() {
    // Test with binary data (not just ASCII)
    let data: &[u8] = &[0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
    let hash = md5::compute(data);
    assert_eq!(hash, "a7ade6f11cc9c0580eca571bef517069");
}
