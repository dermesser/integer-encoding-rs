# integer-encoding-rs

This crate provides encoding and decoding of integers to and from bytestring
representations.

The format is the same -- and compatible with --
[Google's protobuf integer encoding technique](https://developers.google.com/protocol-buffers/docs/encoding).

## FixedInt

`FixedInt` basically executes a `memcpy()` from integer values to bytestrings
and back. Formally, it uses little-endian byte order for encoded values --
however, this crate does not check the architecture for its byte order (so if
you run on non-x86 or armel or any other little-endian architecture, this might
cause issues).

## VarInt

`VarInt` encodes integers in blocks of 7 bits; the MSB is set for every byte but
the last, in which it is cleared.

Signed values are first converted to an unsigned representation using zigzag
encoding (also described on the page linked above), and then encoded as every
other unsigned number.

