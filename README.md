# integer-encoding-rs

[![GitHub repo](https://img.shields.io/badge/github-dermesser/integer--encoding-8da0cb?logo=github)](https://github.com/dermesser/integer-encoding-rs)
[![crates.io version](https://img.shields.io/crates/v/integer-encoding)](https://crates.io/crates/integer-encoding)
[![crate usage](https://img.shields.io/crates/d/integer-encoding)](https://crates.io/crates/integer-encoding)
[![docs.rs status](https://img.shields.io/docsrs/integer-encoding)](https://docs.rs/integer-encoding)
[![crates.io license](https://img.shields.io/crates/l/integer-encoding)](https://github.com/dermesser/integer-encoding-rs/blob/main/LICENSE)
[![CI build status](https://github.com/dermesser/integer-encoding-rs/actions/workflows/test.yml/badge.svg)](https://github.com/dermesser/integer-encoding-rs/actions)

[full documentation](https://docs.rs/integer-encoding/)

This crate provides encoding and decoding of integers to and from bytestring
representations.

The format is described here: [Google's protobuf integer encoding technique](https://developers.google.com/protocol-buffers/docs/encoding).

Please feel free to use `cargo bench` to determine the rate at which your
machine can encode and decode varints and fixedints. Note that one iteration
comprises each eight rounds of encoding (or decoding) a signed and an unsigned
integer each -- divide the resulting benchmark time by 16 in order to have a
rough estimate of time per operation. The integers are very large, so the
results represent the worst case.

## Crate

If you use Tokio v0.2 and you use the asynchronous types in this crate (feature
`tokio_async`), you may be interested in the `v2.0` branch. It is still
maintained with the occasional fix for edge cases and depends on Tokio v0.2.

## FixedInt

`FixedInt` casts integers to bytes by either copying the underlying memory or
performing a transmutation. The encoded values use are little-endian.

However, a trait method is implemented for all integer types allowing convenient conversion between
little and big endian. That is, if you receive a big-endian on the wire and decode it, it will first
be interpreted as little-endian; converting will recover the correct value.

## VarInt

`VarInt` encodes integers in blocks of 7 bits; the MSB is set for every byte but
the last, in which it is cleared.

Signed values are first converted to an unsigned representation using zigzag
encoding (also described on the page linked above), and then encoded as every
other unsigned number.
