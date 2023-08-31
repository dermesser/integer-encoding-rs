use std::io::{Result, Write};

use crate::fixed::FixedInt;
use crate::varint::VarInt;

#[cfg(feature = "tokio_async")]
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[cfg(feature = "futures_async")]
use futures_util::{io::AsyncWrite, io::AsyncWriteExt};

/// A trait for writing integers in VarInt encoding to any `Write` type. This packs encoding and
/// writing into one step.
pub trait VarIntWriter {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize>;
}

/// Like VarIntWriter, but asynchronous.
#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
#[async_trait::async_trait(?Send)]
pub trait VarIntAsyncWriter {
    /// Write a VarInt integer to an asynchronous writer.
    async fn write_varint_async<VI: VarInt>(&mut self, n: VI) -> Result<usize>;
}

#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
#[async_trait::async_trait(?Send)]
impl<AW: AsyncWrite + Unpin> VarIntAsyncWriter for AW {
    async fn write_varint_async<VI: VarInt>(&mut self, n: VI) -> Result<usize> {
        let mut buf = [0_u8; 10];
        let b = n.encode_var(&mut buf);
        self.write_all(&buf[0..b]).await?;
        Ok(b)
    }
}

impl<Inner: Write> VarIntWriter for Inner {
    fn write_varint<VI: VarInt>(&mut self, n: VI) -> Result<usize> {
        let mut buf = [0_u8; 10];
        let used = n.encode_var(&mut buf[..]);

        self.write_all(&buf[0..used])?;
        Ok(used)
    }
}

/// A trait for writing integers without encoding (i.e. `FixedInt`) to any `Write` type.
pub trait FixedIntWriter {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize>;
}

#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
#[async_trait::async_trait(?Send)]
pub trait FixedIntAsyncWriter {
    async fn write_fixedint_async<FI: FixedInt>(&mut self, n: FI) -> Result<usize>;
}

#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
#[async_trait::async_trait(?Send)]
impl<AW: AsyncWrite + Unpin> FixedIntAsyncWriter for AW {
    async fn write_fixedint_async<FI: FixedInt>(&mut self, n: FI) -> Result<usize> {
        let mut buf = [0_u8; 8];
        n.encode_fixed(&mut buf[..std::mem::size_of::<FI>()]);
        self.write_all(&buf[..std::mem::size_of::<FI>()]).await?;
        Ok(std::mem::size_of::<FI>())
    }
}

impl<W: Write> FixedIntWriter for W {
    fn write_fixedint<FI: FixedInt>(&mut self, n: FI) -> Result<usize> {
        let mut buf = [0_u8; 8];
        n.encode_fixed(&mut buf[..std::mem::size_of::<FI>()]);

        self.write_all(&buf[..std::mem::size_of::<FI>()])?;
        Ok(std::mem::size_of::<FI>())
    }
}
