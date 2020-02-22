
use std::fs;
use std::io;

use integer_encoding::*;
use tokio::prelude::*;

fn write_test_file() -> io::Result<()> {
    let mut f = fs::OpenOptions::new().create(true).write(true).open("/tmp/varintbytes")?;
    f.write_varint(30)?;
    f.write_varint(60)?;
    f.write_varint(90)?;
    f.write_varint(9000000)?;
    Ok(())
}

async fn read_and_verify() -> io::Result<()> {
    let mut f = tokio::fs::File::open("/tmp/varintbytes").await?;
    let i1: u32 = f.read_varint_async().await?;
    let i2: u32 = f.read_varint_async().await?;
    let i3: u32 = f.read_varint_async().await?;
    assert!(f.read_varint_async().await.is_err());

    Ok(())
}

#[tokio::main]
async fn main() {
    write_test_file().unwrap();

    read_and_verify().await;
}
