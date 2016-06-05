#[cfg(test)]
mod tests {
    use fixed::FixedInt;

    #[test]
    fn test_u32_enc() {
        let result = (32 as u32).encode_fixed_vec();
        assert_eq!(result, vec![32, 0, 0, 0]);
    }
    #[test]
    fn test_u16_enc() {
        let result = (256 as u16).encode_fixed_vec();
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_i16_enc() {
        let result = (-32768 as i16).encode_fixed_vec();
        assert_eq!(result, vec![0, 128]);
    }
    #[test]
    fn test_i32_enc() {
        let result = (-32767 as i32).encode_fixed_vec();
        assert_eq!(result, vec![1, 128, 255, 255]);
    }
    #[test]
    fn test_all_identity() {
        let a: u16 = 17;
        let b: u32 = 17;
        let c: u64 = 17;
        let d: i16 = -17;
        let e: i32 = -17;
        let f: i64 = -17;

        assert_eq!(a, FixedInt::decode_fixed_vec(&a.encode_fixed_vec()));
        assert_eq!(b, FixedInt::decode_fixed_vec(&b.encode_fixed_vec()));
        assert_eq!(c, FixedInt::decode_fixed_vec(&c.encode_fixed_vec()));
        assert_eq!(d, FixedInt::decode_fixed_vec(&d.encode_fixed_vec()));
        assert_eq!(e, FixedInt::decode_fixed_vec(&e.encode_fixed_vec()));
        assert_eq!(f, FixedInt::decode_fixed_vec(&f.encode_fixed_vec()));
    }
}
