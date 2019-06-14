use std::io;
use std::io::{Read, Result};

use fixed::FixedInt;
use varint::{VarInt, MSB};

/// A trait for reading VarInts from any other `Reader`.
///
/// It's recommended to use a buffered reader, as many small reads will happen.
pub trait VarIntReader {
    /// Returns either the decoded integer, or an error.
    ///
    /// In general, this always reads a whole varint. If the encoded varint's value is bigger
    /// than the valid value range of `VI`, then the value is truncated.
    ///
    /// On EOF, an io::Error with io::ErrorKind::UnexpectedEof is returned.
    fn read_varint<VI: VarInt>(&mut self) -> Result<VI>;
}

impl<R: Read> VarIntReader for R {
    fn read_varint<VI: VarInt>(&mut self) -> Result<VI> {
        const BUFLEN: usize = 10;
        let mut buf = [0 as u8; BUFLEN];
        let mut i = 0;

        loop {
            if i >= BUFLEN {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unterminated varint",
                ));
            }

            let read = try!(self.read(&mut buf[i..i + 1]));

            // EOF
            if read == 0 && i == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached EOF"));
            }

            if buf[i] & MSB == 0 {
                break;
            }

            i += 1;
        }

        let (result, _) = VI::decode_var(&buf[0..i + 1]);

        Ok(result)
    }
}

/// A trait for reading FixedInts from any other `Reader`.
pub trait FixedIntReader {
    /// Read a fixed integer from a reader. How many bytes are read depends on `FI`.
    ///
    /// On EOF, an io::Error with io::ErrorKind::UnexpectedEof is returned.
    fn read_fixedint<FI: FixedInt>(&mut self) -> Result<FI>;
}

impl<R: Read> FixedIntReader for R {
    fn read_fixedint<FI: FixedInt>(&mut self) -> Result<FI> {
        let mut buf = [0 as u8; 8];

        let read = try!(self.read(&mut buf[0..FI::required_space()]));

        if read == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached EOF"));
        }

        Ok(FI::decode_fixed(&buf[0..read]))
    }
}
