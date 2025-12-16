//! VCDIFF address cache implementation
//!
//! Handles the "near" and "same" address caching modes defined in RFC 3284.

use crate::xdelta::{
    constants::{VCD_MODE_HERE, VCD_MODE_SELF},
    parser::VcdiffParser,
};
use stitchr_core::Result;

pub struct AddressCache {
    near_size: usize,
    same_size: usize,
    near: Vec<u64>,
    same: Vec<u64>,
    next_near_slot: usize,
}

impl AddressCache {
    pub fn new(near_size: usize, same_size: usize) -> Self {
        Self {
            near_size,
            same_size,
            near: vec![0; near_size],
            same: vec![0; same_size * 256],
            next_near_slot: 0,
        }
    }

    pub fn near_size(&self) -> usize {
        self.near_size
    }

    pub fn reset(&mut self) {
        self.next_near_slot = 0;
        self.near.fill(0);
        self.same.fill(0);
    }

    pub fn update(&mut self, address: u64) {
        if self.near_size > 0 {
            self.near[self.next_near_slot] = address;
            self.next_near_slot = (self.next_near_slot + 1) % self.near_size;
        }

        if self.same_size > 0 {
            self.same[(address as usize) % (self.same_size * 256)] = address;
        }
    }

    pub fn get_near(&self, index: usize) -> u64 {
        self.near[index]
    }

    pub fn get_same(&self, index: usize) -> u64 {
        self.same[index]
    }
}

impl Default for AddressCache {
    fn default() -> Self {
        Self::new(4, 3) // Default per RFC 3284
    }
}

pub fn decode_address(
    cache: &mut AddressCache,
    stream: &mut VcdiffParser,
    here: u64,
    mode: u8,
) -> Result<u64> {
    let address: u64;

    if mode == VCD_MODE_SELF {
        address = stream.read_7bit_encoded_int()?;
    } else if mode == VCD_MODE_HERE {
        address = here - stream.read_7bit_encoded_int()?;
    } else if (mode as usize - 2) < cache.near_size() {
        // Near cache
        address = cache.get_near((mode as usize) - 2) + stream.read_7bit_encoded_int()?;
    } else {
        // Same cache
        let m = (mode as usize) - (2 + cache.near_size());
        address = cache.get_same(m * 256 + stream.read_u8()? as usize);
    }

    cache.update(address);
    Ok(address)
}
