# xid

[![CI](https://github.com/kazk/xid-rs/workflows/CI/badge.svg)](https://github.com/kazk/xid-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/xid.svg)](https://crates.io/crates/xid)
[![API reference](https://docs.rs/xid/badge.svg)](https://docs.rs/xid/)

Globally unique sortable id generator. A Rust port of https://github.com/rs/xid.

The binary representation is compatible with the Mongo DB 12-byte [ObjectId][object-id].
The value consists of:

- a 4-byte timestamp value in seconds since the Unix epoch
- a 3-byte value based on the machine identifier
- a 2-byte value based on the process id
- a 3-byte incrementing counter, initialized to a random value

The string representation is 20 bytes, using a base32 hex variant with characters `[0-9a-v]`
to retain the sortable property of the id.

See the original [`xid`] project for more details.

## Usage

```rust
println!("{}", xid::new().to_string()); //=> bva9lbqn1bt68k8mj62g
```

## Examples

- [`cargo run --example gen`](./examples/gen.rs): Generate xid

[`xid`]:  https://github.com/rs/xid
[object-id]: https://docs.mongodb.org/manual/reference/object-id/
