//! APS N64 integration tests

#![cfg(feature = "aps")]
mod aps_n64 {
    mod apply;
    mod checksum_validation_tests;
    mod metadata_tests;
    mod validate_tests;
    mod verify_tests;
}
