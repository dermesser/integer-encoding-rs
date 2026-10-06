//! The futures returned by the async traits must be `Send` whenever the reader or writer is.
#![cfg(any(feature = "tokio_async", feature = "futures_async"))]

use std::io::Result;

#[cfg(feature = "tokio_async")]
use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "futures_async")]
use futures_util::io::{AsyncRead, AsyncWrite};

use integer_encoding::*;

fn assert_send<F: Send>(_: F) {}

// Generic helpers in the style of ref-fvm's CAR code, which needed `Send` futures.
async fn read_len<R: AsyncRead + Unpin>(mut reader: &mut R) -> Result<usize> {
    VarIntAsyncReader::read_varint_async(&mut reader).await
}

async fn write_len<W: AsyncWrite + Unpin>(writer: &mut W, bytes: &[u8]) -> Result<usize> {
    writer.write_varint_async(bytes.len()).await?;
    writer.write_fixedint_async(bytes.len() as u32).await
}

fn generic<R, W, I>(reader: &mut R, writer: &mut W, n: I)
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
    I: VarInt + FixedInt + Send,
{
    assert_send(reader.read_varint_async::<I>());
    assert_send(reader.read_fixedint_async::<I>());
    assert_send(writer.write_varint_async(n));
    assert_send(writer.write_fixedint_async(n));
    assert_send(read_len(reader));
    assert_send(write_len(writer, &[1, 2, 3]));
}

#[test]
fn test_async_futures_are_send() {
    let mut reader: &[u8] = &[];
    let mut writer = Vec::new();

    assert_send(reader.read_varint_async::<u64>());
    assert_send(reader.read_fixedint_async::<u64>());
    assert_send(writer.write_varint_async(1_u64));
    assert_send(writer.write_fixedint_async(1_u64));
    assert_send(read_len(&mut reader));
    assert_send(write_len(&mut writer, &[1, 2, 3]));

    generic(&mut reader, &mut writer, 1_u64);
}
