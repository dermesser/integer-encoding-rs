use std::fs;
use std::io;

use integer_encoding::*;

fn write_test_files() -> io::Result<()> {
    let _ = fs::remove_file("/tmp/varintbytes");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("/tmp/varintbytes")?;
    f.write_varint(30 as u32)?;
    f.write_varint(60 as u32)?;
    f.write_varint(90 as u32)?;
    f.write_varint(9000000 as u32)?;

    let _ = fs::remove_file("/tmp/fixedintbytes");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("/tmp/fixedintbytes")?;
    f.write_fixedint(30 as u32)?;
    f.write_fixedint(60 as u32)?;
    f.write_fixedint(90 as u32)?;
    f.write_fixedint(9000000 as u32)?;
    Ok(())
}

async fn read_and_verify_varints() -> io::Result<()> {
    let f = tokio::fs::File::open("/tmp/varintbytes").await?;
    let mut f = tokio::io::BufReader::new(f);
    let i1: u32 = f.read_varint_async().await?;
    let i2: u32 = f.read_varint_async().await?;
    let i3: u32 = f.read_varint_async().await?;
    let i4: u32 = f.read_varint_async().await?;
    assert!(f.read_varint_async::<u32>().await.is_err());
    println!("{:?}", (i1, i2, i3, i4));
    assert!(i2 == 2 * i1 && i3 == 3 * i1);
    Ok(())
}

async fn read_and_verify_fixedints() -> io::Result<()> {
    let f = tokio::fs::File::open("/tmp/fixedintbytes").await?;
    let mut f = tokio::io::BufReader::new(f);
    let i1: u32 = f.read_fixedint_async().await?;
    let i2: u32 = f.read_fixedint_async().await?;
    let i3: u32 = f.read_fixedint_async().await?;
    let i4: u32 = f.read_fixedint_async().await?;
    assert!(f.read_fixedint_async::<u32>().await.is_err());
    println!("{:?}", (i1, i2, i3, i4));
    assert!(i2 == 2 * i1 && i3 == 3 * i1);
    Ok(())
}

#[tokio::main]
async fn main() {
    write_test_files().unwrap();

    read_and_verify_varints().await.unwrap();
    read_and_verify_fixedints().await.unwrap();
}
