#[cfg(test)]
mod tests {
    use reader::VarIntReader;
    use varint::VarInt;
    use writer::VarIntWriter;

    #[test]
    fn test_required_space() {
        assert_eq!((0 as u32).required_space(), 1);
        assert_eq!((1 as u32).required_space(), 1);
        assert_eq!((128 as u32).required_space(), 2);
        assert_eq!((16384 as u32).required_space(), 3);
        assert_eq!((2097151 as u32).required_space(), 3);
        assert_eq!((2097152 as u32).required_space(), 4);
    }

    #[test]
    fn test_encode_u64() {
        assert_eq!((0 as u32).encode_var_vec(), vec![0b00000000]);
        assert_eq!((300 as u32).encode_var_vec(), vec![0b10101100, 0b00000010]);
    }

    #[test]
    fn test_identity_u64() {
        for i in 1 as u64..100 {
            assert_eq!(u64::decode_var_vec(&i.encode_var_vec()), (i, 1));
        }
        for i in 16400 as u64..16500 {
            assert_eq!(u64::decode_var_vec(&i.encode_var_vec()), (i, 3));
        }
    }

    #[test]
    fn test_decode_max_u64() {
        let max_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(u64::decode_var_vec(&max_vec_encoded).0, u64::max_value());
    }

    #[test]
    fn test_encode_i64() {
        assert_eq!((0 as i64).encode_var_vec(), (0 as u32).encode_var_vec());
        assert_eq!((150 as i64).encode_var_vec(), (300 as u32).encode_var_vec());
        assert_eq!(
            (-150 as i64).encode_var_vec(),
            (299 as u32).encode_var_vec()
        );
        assert_eq!(
            (-2147483648 as i64).encode_var_vec(),
            (4294967295 as u64).encode_var_vec()
        );
        assert_eq!(
            (i64::max_value() as i64).encode_var_vec(),
            &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
        assert_eq!(
            (i64::min_value() as i64).encode_var_vec(),
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
    }

    #[test]
    fn test_decode_min_i64() {
        let min_vec_encoded = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(i64::decode_var_vec(&min_vec_encoded).0, i64::min_value());
    }

    #[test]
    fn test_decode_max_i64() {
        let max_vec_encoded = vec![0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01];
        assert_eq!(i64::decode_var_vec(&max_vec_encoded).0, i64::max_value());
    }

    #[test]
    fn test_encode_i16() {
        assert_eq!((150 as i16).encode_var_vec(), (300 as u32).encode_var_vec());
        assert_eq!(
            (-150 as i16).encode_var_vec(),
            (299 as u32).encode_var_vec()
        );
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
}
