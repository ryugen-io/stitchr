//! VCDIFF parser primitive

use stitchr_core::{PatchError, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct VcdiffParser<'a> {
    pub cursor: Cursor<&'a [u8]>,
}

impl<'a> VcdiffParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn new_at_offset(data: &'a [u8], offset: u64) -> Self {
        let mut cursor = Cursor::new(data);
        cursor.set_position(offset);
        Self { cursor }
    }

    pub fn position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn is_eof(&self) -> bool {
        self.cursor.position() >= self.cursor.get_ref().len() as u64
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.cursor
            .read_exact(&mut buf)
            .map_err(|_| PatchError::CorruptedData)?;
        Ok(buf[0])
    }

    pub fn read_7bit_encoded_int(&mut self) -> Result<u64> {
        let mut num: u64 = 0;
        let mut bits: u8;
        loop {
            bits = self.read_u8()?;
            num = (num << 7) + (bits & 0x7f) as u64;
            if (bits & 0x80) == 0 {
                break;
            }
        }
        Ok(num)
    }

    pub fn skip(&mut self, amount: u64) -> Result<()> {
        self.cursor
            .seek(SeekFrom::Current(amount as i64))
            .map_err(|_| PatchError::CorruptedData)?;
        Ok(())
    }

    pub fn seek(&mut self, pos: u64) -> Result<()> {
        self.cursor.set_position(pos);
        Ok(())
    }
}
