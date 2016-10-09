use std::io::{Result, Read};

use varint::{MSB, VarInt};
use fixed::FixedInt;

/// A trait for reading VarInts from any other `Reader`.
///
/// It's recommended to use a buffered reader, as many small reads will happen.
pub trait VarIntReader {
    fn read_varint<VI: VarInt>(&mut self) -> Result<VI>;
}

impl<R: Read> VarIntReader for R {
    fn read_varint<VI: VarInt>(&mut self) -> Result<VI> {
        let mut buf = [0 as u8; 8];
        let mut i = 0;

        loop {
            try!(self.read(&mut buf[i..i + 1]));

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
    fn read_fixedint<FI: FixedInt>(&mut self) -> Result<FI>;
}

impl<R: Read> FixedIntReader for R {
    fn read_fixedint<FI: FixedInt>(&mut self) -> Result<FI> {
        let mut buf = [0 as u8; 8];

        let used = try!(self.read(&mut buf[0..FI::required_space()]));

        Ok(FI::decode_fixed(&buf[0..used]))
    }
}
