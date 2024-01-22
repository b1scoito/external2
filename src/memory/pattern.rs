/// Pattern is a trait that allows for searching for a pattern in a process's memory.
///
use color_eyre::eyre::{self, Error};

use super::Memory;

pub trait Pattern {
    fn find(&self, process: &impl Memory) -> Result<usize, Error>;
}

/// A pattern that can be used to search for a sequence of bytes in a process's memory.
pub struct BytePattern {
    bytes: Vec<u8>,
}

impl BytePattern {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl Pattern for BytePattern {
    /// Returns the address of the first occurrence of the pattern in the process's memory.
    fn find(&self, process: &impl Memory) -> Result<usize, Error> {
        let mut buffer = vec![0u8; 4096];
        let mut address = 0;

        loop {
            let bytes_read = process.read_into(address, &mut buffer)?;

            if bytes_read == 0 {
                return Err(eyre::eyre!("Failed to read memory from process"));
            }

            if let Some(index) = buffer
                .windows(self.bytes.len())
                .position(|window| window == self.bytes.as_slice())
            {
                return Ok(address + index);
            }

            address += bytes_read;
        }
    }
}
