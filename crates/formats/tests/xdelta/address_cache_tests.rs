//! xdelta address cache tests

use stitchr_formats::xdelta::address_cache::AddressCache;
use stitchr_formats::xdelta::{
    address_cache::decode_address,
    constants::{VCD_MODE_HERE, VCD_MODE_SELF},
    parser::VcdiffParser,
};

#[test]
fn test_decode_address_self_mode() {
    let mut cache = AddressCache::default();
    let data = [0x05];
    let mut parser = VcdiffParser::new(&data);
    let addr = decode_address(&mut cache, &mut parser, 100, VCD_MODE_SELF).unwrap();
    assert_eq!(addr, 5);
}

#[test]
fn test_decode_address_here_mode() {
    let mut cache = AddressCache::default();
    let data = [0x05];
    let mut parser = VcdiffParser::new(&data);
    // HERE = 100. Val = 5. Addr = 100 - 5 = 95.
    let addr = decode_address(&mut cache, &mut parser, 100, VCD_MODE_HERE).unwrap();
    assert_eq!(addr, 95);
}

#[test]
fn test_cache_default() {
    let cache = AddressCache::default();
    assert_eq!(cache.near_size(), 4);
    // same_size is 3, total size 3*256 = 768
}

#[test]
fn test_cache_update() {
    let mut cache = AddressCache::default();

    // Update with address 100
    cache.update(100);

    // Near[0] should be 100
    assert_eq!(cache.get_near(0), 100);

    // Same[100 % (3*256)] should be 100
    // 100 % 768 = 100
    assert_eq!(cache.get_same(100), 100);

    // Update with address 200
    cache.update(200);
    // Near[1] should be 200
    assert_eq!(cache.get_near(1), 200);
    // Near[0] still 100
    assert_eq!(cache.get_near(0), 100);
}

#[test]
fn test_cache_near_rollover() {
    let mut cache = AddressCache::default(); // near size 4

    cache.update(10); // [0] = 10
    cache.update(20); // [1] = 20
    cache.update(30); // [2] = 30
    cache.update(40); // [3] = 40
    cache.update(50); // [0] = 50 (Rollover)

    assert_eq!(cache.get_near(0), 50);
    assert_eq!(cache.get_near(1), 20);
}

#[test]
fn test_cache_same_overwrite() {
    let mut cache = AddressCache::default(); // same size 3 * 256

    let addr1 = 100;
    let addr2 = 100 + (3 * 256); // Collides with addr1

    cache.update(addr1);
    assert_eq!(cache.get_same(100), addr1);

    cache.update(addr2);
    assert_eq!(cache.get_same(100), addr2);
}
