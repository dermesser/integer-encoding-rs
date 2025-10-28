#[cfg(test)]
mod tests {
    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    use crate::reader::VarIntAsyncReader;
    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    use crate::writer::VarIntAsyncWriter;

    use crate::reader::VarIntReader;
    use crate::varint::VarInt;
    use crate::writer::VarIntWriter;

    #[test]
    fn test_required_space() {
        assert_eq!(0_u32.required_space(), 1);
        assert_eq!(1_u32.required_space(), 1);
        assert_eq!(128_u32.required_space(), 2);
        assert_eq!(16384_u32.required_space(), 3);
        assert_eq!(2097151_u32.required_space(), 3);
        assert_eq!(2097152_u32.required_space(), 4);
    }

    #[test]
    fn test_encode_u64() {
        assert_eq!(0_u32.encode_var_vec(), vec![0b00000000]);
        assert_eq!(300_u32.encode_var_vec(), vec![0b10101100, 0b00000010]);
    }

    #[test]
    fn test_identity_u64() {
        for i in 1_u64..100 {
            assert_eq!(
                u64::decode_var(i.encode_var_vec().as_slice()).unwrap(),
                (i, 1)
            );
        }
        for i in 16400_u64..16500 {
            assert_eq!(
                u64::decode_var(i.encode_var_vec().as_slice()).unwrap(),
                (i, 3)
            );
        }
    }

    #[test]
    fn test_decode_max_u64() {
        let max_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            u64::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            u64::MAX
        );
    }

    #[test]
    fn test_decode_max_u64_plus_one() {
        let max_vec_encoded = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02];
        assert!(u64::decode_var(max_vec_encoded.as_slice()).is_none());
    }

    #[test]
    fn test_encode_i64() {
        assert_eq!(0_i64.encode_var_vec(), 0_u32.encode_var_vec());
        assert_eq!(150_i64.encode_var_vec(), 300_u32.encode_var_vec());
        assert_eq!((-150_i64).encode_var_vec(), 299_u32.encode_var_vec());
        assert_eq!(
            (-2147483648_i64).encode_var_vec(),
            4294967295_u64.encode_var_vec()
        );
        assert_eq!(
            i64::MAX.encode_var_vec(),
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
        assert_eq!(
            i64::MIN.encode_var_vec(),
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn test_decode_min_i64() {
        let min_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            i64::decode_var(min_vec_encoded.as_slice()).unwrap().0,
            i64::MIN
        );
    }

    #[test]
    fn test_decode_max_i64() {
        let max_vec_encoded = vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(
            i64::decode_var(max_vec_encoded.as_slice()).unwrap().0,
            i64::MAX
        );
    }

    #[test]
    fn test_encode_i16() {
        assert_eq!(150_i16.encode_var_vec(), 300_u32.encode_var_vec());
        assert_eq!((-150_i16).encode_var_vec(), 299_u32.encode_var_vec());
    }

    #[test]
    fn test_reader_writer() {
        let mut buf = Vec::with_capacity(128);

        let i1: u32 = 1;
        let i2: u32 = 65532;
        let i3: u32 = 4200123456;
        let i4: i64 = i3 as i64 * 1000;
        let i5: i32 = -32456;

        assert!(buf.write_varint(i1).is_ok());
        assert!(buf.write_varint(i2).is_ok());
        assert!(buf.write_varint(i3).is_ok());
        assert!(buf.write_varint(i4).is_ok());
        assert!(buf.write_varint(i5).is_ok());

        let mut reader: &[u8] = buf.as_ref();

        assert_eq!(i1, reader.read_varint().unwrap());
        assert_eq!(i2, reader.read_varint().unwrap());
        assert_eq!(i3, reader.read_varint().unwrap());
        assert_eq!(i4, reader.read_varint().unwrap());
        assert_eq!(i5, reader.read_varint().unwrap());

        assert!(reader.read_varint::<u32>().is_err());
    }

    #[cfg(any(feature = "tokio_async", feature = "futures_async"))]
    #[tokio::test]
    async fn test_async_reader() {
        let mut buf = Vec::with_capacity(128);

        let i1: u32 = 1;
        let i2: u32 = 65532;
        let i3: u32 = 4200123456;
        let i4: i64 = i3 as i64 * 1000;
        let i5: i32 = -32456;

        buf.write_varint_async(i1).await.unwrap();
        buf.write_varint_async(i2).await.unwrap();
        buf.write_varint_async(i3).await.unwrap();
        buf.write_varint_async(i4).await.unwrap();
        buf.write_varint_async(i5).await.unwrap();

        let mut reader: &[u8] = buf.as_ref();

        assert_eq!(i1, reader.read_varint_async().await.unwrap());
        assert_eq!(i2, reader.read_varint_async().await.unwrap());
        assert_eq!(i3, reader.read_varint_async().await.unwrap());
        assert_eq!(i4, reader.read_varint_async().await.unwrap());
        assert_eq!(i5, reader.read_varint_async().await.unwrap());
        assert!(reader.read_varint_async::<u32>().await.is_err());
    }

    #[test]
    fn test_unterminated_varint() {
        let buf = vec![0xff_u8; 12];
        let mut read = buf.as_slice();
        assert!(read.read_varint::<u64>().is_err());
    }

    #[test]
    fn test_unterminated_varint_2() {
        let buf = [0xff, 0xff];
        let mut read = &buf[..];
        assert!(read.read_varint::<u64>().is_err());
    }

    #[test]
    fn test_decode_extra_bytes_u64() {
        let mut encoded = 0x12345u64.encode_var_vec();
        assert_eq!(u64::decode_var(&encoded[..]), Some((0x12345, 3)));

        encoded.push(0x99);
        assert_eq!(u64::decode_var(&encoded[..]), Some((0x12345, 3)));

        let encoded = [0xFF, 0xFF, 0xFF];
        assert_eq!(u64::decode_var(&encoded[..]), None);

        // Overflow
        let mut encoded = vec![0xFF; 64];
        encoded.push(0x00);
        assert_eq!(u64::decode_var(&encoded[..]), None);
    }

    #[test]
    fn test_decode_extra_bytes_i64() {
        let mut encoded = (-0x12345i64).encode_var_vec();
        assert_eq!(i64::decode_var(&encoded[..]), Some((-0x12345, 3)));

        encoded.push(0x99);
        assert_eq!(i64::decode_var(&encoded[..]), Some((-0x12345, 3)));

        let encoded = [0xFF, 0xFF, 0xFF];
        assert_eq!(i64::decode_var(&encoded[..]), None);

        // Overflow
        let mut encoded = vec![0xFF; 64];
        encoded.push(0x00);
        assert_eq!(i64::decode_var(&encoded[..]), None);
    }

    #[test]
    fn test_regression_22() {
        let encoded: Vec<u8> = 0x112233_u64.encode_var_vec();
        assert_eq!(
            encoded.as_slice().read_varint::<i8>().unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn test_decode_var_too_big() {
        assert_eq!(u8::decode_var(&0x1FF_u64.encode_var_vec()), None, "u8");
        assert_eq!(u16::decode_var(&0x1FFFF_u64.encode_var_vec()), None, "u16");
        assert_eq!(
            u32::decode_var(&0x1FFFFFFFF_u64.encode_var_vec()),
            None,
            "u32"
        );
        assert_eq!(i8::decode_var(&0x80_i64.encode_var_vec()), None, "i8");
        assert_eq!(i16::decode_var(&0x8000_i64.encode_var_vec()), None, "i16");
        assert_eq!(
            i32::decode_var(&0x80000000_i64.encode_var_vec()),
            None,
            "i32"
        );
    }
}
