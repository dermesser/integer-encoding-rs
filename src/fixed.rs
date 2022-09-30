use std::convert::TryInto;
use std::mem::size_of;

/// `FixedInt` provides encoding/decoding to and from fixed int representations. Note that current
/// Rust versions already provide this functionality via the `to_le_bytes()` and `to_be_bytes()`
/// methods.
///
/// The emitted bytestring contains the bytes of the integer in machine endianness.
pub trait FixedInt: Sized + Copy {
    type Bytes: AsRef<[u8]>;
    const ENCODED_SIZE: usize = size_of::<Self>();

    /// Encode a value into the given slice using little-endian. Returns `None` if `dst`
    /// doesn't provide enough space to encode this integer.
    ///
    /// Use `switch_endianness()` if machine endianness doesn't match the desired target encoding.
    fn encode_fixed(self, dst: &mut [u8]) -> Option<()>;
    /// Returns the representation of [`FixedInt`] as [`Bytes`], the little-endian representation
    /// of self in the stack.
    fn encode_fixed_light(self) -> Self::Bytes;

    /// Decode a value from the given slice assuming little-endian. Use `switch_endianness()` on
    /// the returned value if the source was not encoded in little-endian.
    fn decode_fixed(src: &[u8]) -> Option<Self>;

    /// Helper: Encode the value and return a Vec.
    fn encode_fixed_vec(self) -> Vec<u8> {
        self.encode_fixed_light().as_ref().to_vec()
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

            fn encode_fixed(self, dst: &mut [u8]) -> Option<()> {
                if dst.len() == size_of::<Self>() {
                    dst.clone_from_slice(&self.to_le_bytes());
                    Some(())
                } else {
                    None
                }
            }

            fn encode_fixed_light(self) -> Self::Bytes {
                self.to_le_bytes()
            }

            fn decode_fixed(src: &[u8]) -> Option<Self> {
                if src.len() == size_of::<Self>() {
                    Some(Self::from_le_bytes(src.try_into().unwrap()))
                } else {
                    None
                }
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
