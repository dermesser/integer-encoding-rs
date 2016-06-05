
const MSB: u8 = 0b10000000;
const DROP_MSB: u8 = 0b01111111;
const EXTRACT_SEVEN: u8 = DROP_MSB;

#[inline]
fn required_encoded_space_unsigned(mut v: u64) -> usize {
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

// implementable for all ints, basically. Bounds are required for the required_space() default
// implementation.
pub trait VarInt : Sized + Copy {
    // How many bytes this number needs in varint representation
    fn required_space(self) -> usize;
    fn decode_var(&[u8]) -> Self;
    fn encode_var(self, &mut [u8]) -> usize;

    fn decode_var_vec(v: &Vec<u8>) -> Self {
        Self::decode_var(&v)
    }
    fn encode_var_vec(self) -> Vec<u8> {
        let mut v = Vec::new();
        v.resize(self.required_space(), 0);
        self.encode_var(&mut v);
        v
    }
}

#[inline]
fn zigzag_encode(from: i64) -> u64 {
    if from < 0 {
        (2 * -from) as u64 - 1
    } else if from > 0 {
        2 * from as u64
    } else {
        0
    }
}

#[inline]
fn zigzag_decode(from: u64) -> i64 {
    if from % 2 == 0 {
        from as i64 / 2
    } else {
        -((from as i64 + 1) / 2)
    }
}

// Duplicating the decode/encode logic is easier than hacking together default implementations
// with trait bounds.
impl VarInt for u64 {
    fn required_space(self) -> usize {
        required_encoded_space_unsigned(self)
    }

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        result
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

impl VarInt for u32 {
    fn required_space(self) -> usize {
        required_encoded_space_unsigned(self as u64)
    }

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u32 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u32) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        result
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

impl VarInt for u16 {
    fn required_space(self) -> usize {
        required_encoded_space_unsigned(self as u64)
    }

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u16 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u16) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        result
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

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        zigzag_decode(result) as Self
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

impl VarInt for i32 {
    fn required_space(self) -> usize {
        required_encoded_space_signed(self as i64)
    }

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        zigzag_decode(result) as Self
    }

    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n = zigzag_encode(self as i64);
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

impl VarInt for i16 {
    fn required_space(self) -> usize {
        required_encoded_space_signed(self as i64)
    }

    fn decode_var(src: &[u8]) -> Self {
        let mut result: u64 = 0;
        let mut shift = 0;

        for b in src.iter() {
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 {
                break;
            }
        }

        zigzag_decode(result) as Self
    }

    fn encode_var(self, dst: &mut [u8]) -> usize {
        assert!(dst.len() >= self.required_space());
        let mut n = zigzag_encode(self as i64);
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
