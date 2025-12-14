//! IPS integration tests

#![cfg(feature = "ips")]
mod ips {
    mod apply;
    mod checksum_validation;
    mod metadata_tests;
    mod validate_tests;
}
