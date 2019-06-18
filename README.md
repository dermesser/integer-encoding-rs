# integer-encoding-rs

[![crates.io](https://img.shields.io/crates/v/integer-encoding.svg)](https://crates.io/crates/integer-encoding)
[![Build Status](https://travis-ci.org/dermesser/integer-encoding-rs.svg?branch=master)](https://travis-ci.org/dermesser/integer-encoding-rs)

[full documentation](https://docs.rs/integer-encoding/)

This crate provides encoding and decoding of integers to and from bytestring
representations.

The format is described here: [Google's protobuf integer encoding technique](https://developers.google.com/protocol-buffers/docs/encoding).

## FixedInt

`FixedInt` basically executes a `memcpy()` from integer values to bytestrings
and back. Formally, it uses little-endian byte order for encoded values --
however, this crate does not check the architecture for its byte order (so if
you run on a big-endian architecture, this will cause issues).

## VarInt

`VarInt` encodes integers in blocks of 7 bits; the MSB is set for every byte but
the last, in which it is cleared.

Signed values are first converted to an unsigned representation using zigzag
encoding (also described on the page linked above), and then encoded as every
other unsigned number.

