pub const MSB: u8 = 0b10000000;
const DROP_MSB: u8 = 0b01111111;
const EXTRACT_SEVEN: u8 = DROP_MSB;

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

#[inline]
fn required_encoded_space_signed(v: i64) -> usize {
    required_encoded_space_unsigned(zigzag_encode(v))
}

/// Varint (variable length integer) encoding, as described in
/// https://developers.google.com/protocol-buffers/docs/encoding.
/// Uses zigzag encoding (also described there) for signed integer representation.
pub trait VarInt: Sized + Copy {
    /// Returns the number of bytes this number needs in its encoded form.
    fn required_space(self) -> usize;
    /// Decode a value from the slice. Returns the value and the number of bytes read from the
    /// slice (can be used to read several consecutive values from a big slice)
    fn decode_var(&[u8]) -> (Self, usize);
    /// Encode a value into the slice.
    fn encode_var(self, &mut [u8]) -> usize;

    /// Helper: (bit useless) - Decode value from the Vec.
    fn decode_var_vec(v: &Vec<u8>) -> (Self, usize) {
        Self::decode_var(&v)
    }
    /// Helper: Encode a value and return the encoded form as Vec.
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

macro_rules! impl_varint {
    ($t:ty, unsigned) => {
        impl VarInt for $t {
            fn required_space(self) -> usize {
                required_encoded_space_unsigned(self as u64)
            }

            fn decode_var(src: &[u8]) -> (Self, usize) {
                let (n, s) = u64::decode_var(src);
                (n as Self, s)
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

            fn decode_var(src: &[u8]) -> (Self, usize) {
                let (n, s) = i64::decode_var(src);
                (n as Self, s)
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

// Below are the "base implementations" doing the actual encodings; all other integer types are
// first cast to these biggest types before being encoded.

impl VarInt for u64 {
    fn required_space(self) -> usize {
        required_encoded_space_unsigned(self)
    }

    fn decode_var(src: &[u8]) -> (Self, usize) {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 || shift > (10 * 7) {
                break;
            }
        }

        (result, shift / 7 as usize)
    }
    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n = self;
        let mut i = 0;

        if n > 0 {
            while n > 0 {
                dst[i] = MSB | (n as u8 & EXTRACT_SEVEN) as u8;
                i += 1;
                n >>= 7;
            }

            dst[i - 1] = DROP_MSB & dst[i - 1];
            i
        } else {
            dst[0] = 0;
            1
        }
    }
}

impl VarInt for i64 {
    fn required_space(self) -> usize {
        required_encoded_space_signed(self)
    }

    fn decode_var(src: &[u8]) -> (Self, usize) {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 || shift > (10 * 7) {
                break;
            }
        }

        (zigzag_decode(result) as Self, shift / 7 as usize)
    }

    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n: u64 = zigzag_encode(self as i64);
        let mut i = 0;

        if n > 0 {
            while n > 0 {
                dst[i] = MSB | (n as u8 & EXTRACT_SEVEN) as u8;
                i += 1;
                n >>= 7;
            }

            dst[i - 1] = DROP_MSB & dst[i - 1];
            i
        } else {
            dst[0] = 0;
            1
        }
    }
}
