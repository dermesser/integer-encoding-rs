use std::mem::size_of;

/// Most-significant byte, == 0x80
pub const MSB: u8 = 0b1000_0000;
/// All bits except for the most significant. Can be used as bitmask to drop the most-signficant
/// bit using `&` (binary-and).
const DROP_MSB: u8 = 0b0111_1111;

/// How many bytes an integer uses when being encoded as a VarInt.
#[inline]
fn required_encoded_space_unsigned(mut v: u64) -> usize {
    if v == 0 {
        return 1;
    }

    let mut logcounter = 0;
    while v > 0 {
        logcounter += 1;
        v >>= 7;
    }
    logcounter
}

/// How many bytes an integer uses when being encoded as a VarInt.
#[inline]
fn required_encoded_space_signed(v: i64) -> usize {
    required_encoded_space_unsigned(zigzag_encode(v))
}

/// Varint (variable length integer) encoding, as described in
/// https://developers.google.com/protocol-buffers/docs/encoding.
///
/// Uses zigzag encoding (also described there) for signed integer representation.
pub trait VarInt: Sized + Copy {
    /// Returns the number of bytes this number needs in its encoded form. Note: This varies
    /// depending on the actual number you want to encode.
    fn required_space(self) -> usize;
    /// Decode a value from the slice. Returns the value and the number of bytes read from the
    /// slice (can be used to read several consecutive values from a big slice)
    /// return None if all bytes has MSB set.
    fn decode_var(src: &[u8]) -> Option<(Self, usize)>;
    /// Decode a value from the slice, as with `decode_var`, but if `allow_non_minimal_encoding` is false,
    /// then return None if the slice does not contain the minimum representation of the value. For example, if the slice is [152] it
    /// will fail because this can be represented as [24]. Use this function if round trip encoding
    /// is required.
    fn decode_var_exact(src: &[u8], allow_non_minimal_encoding: bool) -> Option<(Self, usize)>;
    /// Encode a value into the slice. The slice must be at least `required_space()` bytes long.
    /// The number of bytes taken by the encoded integer is returned.
    fn encode_var(self, src: &mut [u8]) -> usize;

    /// Helper: Encode a value and return the encoded form as Vec. The Vec must be at least
    /// `required_space()` bytes long.
    fn encode_var_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(self.required_space(), 0);
        self.encode_var(&mut v);
        v
    }
}

#[inline]
fn zigzag_encode(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

// see: http://stackoverflow.com/a/2211086/56332
// casting required because operations like unary negation
// cannot be performed on unsigned integers
#[inline]
fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

pub(crate) trait VarIntMaxSize {
    fn varint_max_size() -> usize;
}

impl<VI: VarInt> VarIntMaxSize for VI {
    fn varint_max_size() -> usize {
        (size_of::<VI>() * 8 + 7) / 7
    }
}

macro_rules! impl_varint {
    ($t:ty, unsigned) => {
        impl VarInt for $t {
            fn required_space(self) -> usize {
                required_encoded_space_unsigned(self as u64)
            }

            fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
                let (n, s) = Self::decode_var_exact(src, true)?;
                Some((n as Self, s))
            }

            fn decode_var_exact(
                src: &[u8],
                allow_non_minimal_encoding: bool,
            ) -> Option<(Self, usize)> {
                let (n, s) = decode_exact::<$t>(src, allow_non_minimal_encoding)?;
                Some((n as Self, s))
            }

            fn encode_var(self, dst: &mut [u8]) -> usize {
                (self as u64).encode_var(dst)
            }
        }
    };
    ($t:ty, signed) => {
        impl VarInt for $t {
            fn required_space(self) -> usize {
                required_encoded_space_signed(self as i64)
            }

            fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
                let (n, s) = Self::decode_var_exact(src, true)?;
                Some((n as Self, s))
            }

            fn decode_var_exact(
                src: &[u8],
                allow_non_minimal_encoding: bool,
            ) -> Option<(Self, usize)> {
                let (n, s) = decode_exact::<$t>(src, allow_non_minimal_encoding)?;
                Some((zigzag_decode(n) as Self, s))
            }

            fn encode_var(self, dst: &mut [u8]) -> usize {
                (self as i64).encode_var(dst)
            }
        }
    };
}

impl_varint!(usize, unsigned);
impl_varint!(u32, unsigned);
impl_varint!(u16, unsigned);
impl_varint!(u8, unsigned);

impl_varint!(isize, signed);
impl_varint!(i32, signed);
impl_varint!(i16, signed);
impl_varint!(i8, signed);

fn decode_exact<VI>(src: &[u8], allow_non_minimal_encoding: bool) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;

    //let max_shift : u32= (VI::varint_max_size() as u32 - 1) * 7;
    let mut success = false;
    for b in src.iter() {
        let msb_dropped = b & DROP_MSB;
        // Check if there is space to shift so that we don't overflow
        if (msb_dropped as u64).leading_zeros() - ((size_of::<u64>() - size_of::<VI>()) as u32 * 8)
            < shift
        {
            success = false;
            break;
        }
        result |= (msb_dropped as u64) << shift;
        shift += 7;

        // a zero byte is only allowed if it is the first byte
        if !allow_non_minimal_encoding && *b == 0 && shift != 0 {
            success = false;
            break;
        }
        if b & MSB == 0 || shift > (9 * 7) {
            success = b & MSB == 0;
            break;
        }
    }

    if success {
        Some((result, shift as usize / 7 as usize))
    } else {
        None
    }
}

// Below are the "base implementations" doing the actual encodings; all other integer types are
// first cast to these biggest types before being encoded.

impl VarInt for u64 {
    fn required_space(self) -> usize {
        required_encoded_space_unsigned(self)
    }

    #[inline]
    fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
        Self::decode_var_exact(src, true)
    }

    #[inline]
    fn decode_var_exact(src: &[u8], allow_non_minimal_encoding: bool) -> Option<(Self, usize)> {
        decode_exact::<u64>(src, allow_non_minimal_encoding)
    }

    #[inline]
    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n = self;
        let mut i = 0;

        while n >= 0x80 {
            dst[i] = MSB | (n as u8);
            i += 1;
            n >>= 7;
        }

        dst[i] = n as u8;
        i + 1
    }
}

impl VarInt for i64 {
    fn required_space(self) -> usize {
        required_encoded_space_signed(self)
    }

    #[inline]
    fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
        if let Some((result, size)) = u64::decode_var(src) {
            Some((zigzag_decode(result) as Self, size))
        } else {
            None
        }
    }

    #[inline]
    fn decode_var_exact(src: &[u8], allow_non_minimal_encoding: bool) -> Option<(Self, usize)> {
        if let Some((result, size)) = u64::decode_var_exact(src, allow_non_minimal_encoding) {
            Some((zigzag_decode(result) as Self, size))
        } else {
            None
        }
    }

    #[inline]
    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n: u64 = zigzag_encode(self);
        let mut i = 0;

        while n >= 0x80 {
            dst[i] = MSB | (n as u8);
            i += 1;
            n >>= 7;
        }

        dst[i] = n as u8;
        i + 1
    }
}
