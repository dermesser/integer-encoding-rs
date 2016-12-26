use std::io::{Result, Write};

use fixed::FixedInt;
use varint::VarInt;

/// A trait for writing integers in VarInt encoding to any `Write` type.
pub trait VarIntWriter {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize>;
}

impl<Inner: Write> VarIntWriter for Inner {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize> {
        let mut buf = [0 as u8; 10];
        let used = n.encode_var(&mut buf[..]);

        self.write(&buf[0..used])
    }
}

/// A trait for writing integers without encoding (i.e. `FixedInt`) to any `Write` type.
pub trait FixedIntWriter {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize>;
}

impl<W: Write> FixedIntWriter for W {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize> {
        let mut buf = [0 as u8; 8];
        n.encode_fixed(&mut buf[0..FI::required_space()]);

        self.write(&buf[0..FI::required_space()])
    }
}
