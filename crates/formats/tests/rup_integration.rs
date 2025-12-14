//! RUP integration tests

#![cfg(feature = "rup")]
mod rup {
    mod apply;
    mod checksum_validation_tests;
    mod metadata_tests;
    mod validate_tests;
    mod varint_tests;
}
