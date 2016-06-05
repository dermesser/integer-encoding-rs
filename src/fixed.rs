
use std::mem::transmute;

/// FixedInt provides encoding/decoding to and from fixed int representations.
/// The emitted bytestring contains the bytes of the integer in little-endian order.
pub trait FixedInt : Sized + Copy {
    /// Returns how many bytes are required to represent the given type.
    fn size_required() -> usize;
    /// Encode a value into the given slice.
    fn encode_fixed(self, &mut [u8]);
    /// Decode a value from the given slice.
    fn decode_fixed(&[u8]) -> Self;

    /// Helper: Encode the value and return a Vec.
    fn encode_fixed_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(Self::size_required(), 0);
        self.encode_fixed(&mut v[..]);
        v
    }
    /// Helper: Decode the value from the Vec.
    fn decode_fixed_vec(v: &Vec<u8>) -> Self {
        assert_eq!(v.len(), Self::size_required());
        Self::decode_fixed(&v[..])
    }
}

impl FixedInt for u64 {
    fn size_required() -> usize {
        8
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<u64, [u8; 8]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> u64 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 8] = [0; 8];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 8], u64>(arr) };
        dec
    }
}

impl FixedInt for u32 {
    fn size_required() -> usize {
        4
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<u32, [u8; 4]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> u32 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 4] = [0; 4];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 4], u32>(arr) };
        dec
    }
}

impl FixedInt for u16 {
    fn size_required() -> usize {
        2
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<u16, [u8; 2]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> u16 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 2] = [0; 2];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 2], u16>(arr) };
        dec
    }
}

impl FixedInt for i64 {
    fn size_required() -> usize {
        8
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<i64, [u8; 8]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> i64 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 8] = [0; 8];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 8], i64>(arr) };
        dec
    }
}

impl FixedInt for i32 {
    fn size_required() -> usize {
        4
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<i32, [u8; 4]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> i32 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 4] = [0; 4];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 4], i32>(arr) };
        dec
    }
}

impl FixedInt for i16 {
    fn size_required() -> usize {
        2
    }

    fn encode_fixed(self, dst: &mut [u8]) {
        assert_eq!(dst.len(), Self::size_required());
        let enc = unsafe { transmute::<i16, [u8; 2]>(self) };
        dst.clone_from_slice(&enc);
    }
    fn decode_fixed(src: &[u8]) -> i16 {
        assert_eq!(src.len(), Self::size_required());
        let mut arr: [u8; 2] = [0; 2];

        for i in 0..src.len() {
            arr[i] = src[i];
        }

        let dec = unsafe { transmute::<[u8; 2], i16>(arr) };
        dec
    }
}
