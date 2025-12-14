//! BPS integration tests

#![cfg(feature = "bps")]
mod bps {
    mod apply;
    mod checksum_validation;
    mod metadata_tests;
    mod validate_tests;
    mod varint_tests;
    mod verify_tests;
}
