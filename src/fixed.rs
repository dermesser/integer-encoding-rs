use std::convert::TryInto;
use std::mem::size_of;

/// `FixedInt` provides encoding/decoding to and from fixed int representations.
///
/// The emitted bytestring contains the bytes of the integer in machine endianness.
pub trait FixedInt: Sized + Copy {
    type Bytes: AsRef<[u8]>;

    /// Encode a value into the given slice. `dst` must be exactly `REQUIRED_SPACE` bytes.
    fn encode_fixed(self, dst: &mut [u8]);
    /// Decode a value from the given slice. `src` must be exactly `REQUIRED_SPACE` bytes.
    fn decode_fixed(src: &[u8]) -> Self;
    /// Returns the representation of [`FixedInt`] as [`Bytes`], the little-endian representation
    /// of self in the stack.
    fn encode_fixed_light(self) -> Self::Bytes;

    /// Helper: Encode the value and return a Vec.
    fn encode_fixed_vec(self) -> Vec<u8> {
        self.encode_fixed_light().as_ref().to_vec()
    }

    /// Helper: Decode the value from the Vec.
    fn decode_fixed_vec(v: &Vec<u8>) -> Self {
        Self::decode_fixed(&v[..])
    }

    /// integer-encoding-rs always emits and receives little-endian integers (converting implicitly
    /// on big-endian machines). If you receive a big-endian integer, and would like it to be
    /// treated correctly, use this helper method to convert between endiannesses.
    fn switch_endianness(self) -> Self;
}

macro_rules! impl_fixedint {
    ($t:ty) => {
        impl FixedInt for $t {
            type Bytes = [u8; size_of::<$t>()];

            fn encode_fixed_light(self) -> Self::Bytes {
                self.to_le_bytes()
            }

            fn encode_fixed(self, dst: &mut [u8]) {
                assert_eq!(dst.len(), size_of::<Self>());
                dst.clone_from_slice(&self.to_le_bytes());
            }

            #[cfg(target_endian = "little")]
            fn decode_fixed(src: &[u8]) -> Self {
                Self::from_le_bytes(src.try_into().unwrap())
            }

            #[cfg(target_endian = "big")]
            fn decode_fixed(src: &[u8]) -> $t {
                Self::from_be_bytes(src.try_into().unwrap())
            }

            fn switch_endianness(self) -> Self {
                Self::from_le_bytes(self.to_be_bytes())
            }
        }
    };
}

impl_fixedint!(usize);
impl_fixedint!(u64);
impl_fixedint!(u32);
impl_fixedint!(u16);
impl_fixedint!(u8);
impl_fixedint!(isize);
impl_fixedint!(i64);
impl_fixedint!(i32);
impl_fixedint!(i16);
impl_fixedint!(i8);
