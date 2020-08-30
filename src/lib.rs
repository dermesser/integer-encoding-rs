//! Fast serialization of integers.
//!
//! ```
//! use integer_encoding::*;
//!
//! fn main() {
//!     let a: u32 = 344;
//!     let encoded_byte_slice = a.encode_fixed_light();
//!     assert_eq!(a, u32::decode_fixed(encoded_byte_slice));
//!     assert_eq!(4, encoded_byte_slice.len());
//!
//!     let b: i32 = -111;
//!     let encoded_byte_vec = b.encode_var_vec();
//!     assert_eq!(Some((b, 2)), i32::decode_var(&encoded_byte_vec));
//! }
//! ```

mod fixed;
mod fixed_tests;

mod varint;
mod varint_tests;

mod reader;
mod writer;

pub use fixed::FixedInt;
pub use varint::VarInt;

#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
pub use reader::FixedIntAsyncReader;
pub use reader::FixedIntReader;
#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
pub use reader::VarIntAsyncReader;
pub use reader::VarIntReader;

#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
pub use writer::FixedIntAsyncWriter;
pub use writer::FixedIntWriter;
#[cfg(any(feature = "tokio_async", feature = "futures_async"))]
pub use writer::VarIntAsyncWriter;
pub use writer::VarIntWriter;
