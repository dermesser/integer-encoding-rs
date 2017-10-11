
use std::mem::transmute;

/// FixedInt provides encoding/decoding to and from fixed int representations.
/// The emitted bytestring contains the bytes of the integer in little-endian order.
pub trait FixedInt: Sized + Copy {
    /// Returns how many bytes are required to represent the given type.
    fn required_space() -> usize;
    /// Encode a value into the given slice.
    fn encode_fixed(self, &mut [u8]);
    /// Decode a value from the given slice.
    fn decode_fixed(&[u8]) -> Self;

    /// Helper: Encode the value and return a Vec.
    fn encode_fixed_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(Self::required_space(), 0);
        self.encode_fixed(&mut v[..]);
        v
    }
    /// Helper: Decode the value from the Vec.
    fn decode_fixed_vec(v: &Vec<u8>) -> Self {
        assert_eq!(v.len(), Self::required_space());
        Self::decode_fixed(&v[..])
    }
}

macro_rules! impl_fixedint {
    ($t:ty, $sz:expr) => {
        impl FixedInt for $t {
            fn required_space() -> usize {
                $sz
            }
            fn encode_fixed(self, dst: &mut [u8]) {
                assert_eq!(dst.len(), Self::required_space());
                let encoded = unsafe { transmute::<&$t, &[u8; $sz]>(&self) };
                dst.clone_from_slice(encoded);
            }
            fn decode_fixed(src: &[u8]) -> $t {
                assert_eq!(src.len(), Self::required_space());
                let mut decoded: $t = 0;
                let cast = unsafe { transmute::<&mut $t, &mut [u8; $sz]>(&mut decoded) };
                for i in 0..$sz {
                    cast[i] = src[i];
                }
                decoded
            }
        }
    }
}

impl_fixedint!(usize, 8);
impl_fixedint!(u64, 8);
impl_fixedint!(u32, 4);
impl_fixedint!(u16, 2);
impl_fixedint!(isize, 8);
impl_fixedint!(i64, 8);
impl_fixedint!(i32, 4);
impl_fixedint!(i16, 2);
