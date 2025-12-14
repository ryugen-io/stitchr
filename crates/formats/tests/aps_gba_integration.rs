//! APS GBA integration tests

#![cfg(feature = "aps")]
mod aps_gba {
    mod apply;
    mod checksum_validation_tests;
    mod metadata_tests;
    mod validate_tests;
    mod verify_tests;
}
