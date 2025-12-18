# ez-hash

Ergonomic hashing helpers for Rust.

- Single, simple API: each hash algorithm is a single function.
- Supports hashing either a single value or multiple parts by implementing `Hashable`.
- Common hash algorithms behind Cargo features (default: all enabled).

## Installation

```toml
[dependencies]
ez-hash = "1.0"
```

To enable only selected algorithms:

```toml
ez-hash = { version = "1.0", default-features = false, features = ["sha2", "sha3"] }
```

## Usage

Single-part:

```rust
use ez_hash::sha256;

let digest = sha256("hello");
```

Multi-part (tuple / array / vec):

```rust
use ez_hash::{keccak256, sha256};

let a = sha256(("a", "bc"));
let b = sha256(["a", "b", "c"]);
let c = keccak256(("prefix", "data", "suffix"));
assert_eq!(a, b);
```

Bytes:

```rust
use ez_hash::sha256;

let bytes = 1u32.to_le_bytes();
let digest = sha256(bytes);
```

## API

The crate exposes:

- `Hashable` trait (implemented for common byte/string types and for tuples/arrays/vectors of hashables)

Hash functions (availability depends on enabled features):

- SHA1: `sha1`
- SHA2: `sha224`, `sha256`, `sha384`, `sha512`, `sha512_224`, `sha512_256`
- SHA3: `sha3_224`, `sha3_256`, `sha3_384`, `sha3_512`
- Keccak: `keccak256`, `keccak512`
- Blake2: `blake2b_256`, `blake2b_384`, `blake2b_512`, `blake2s_128`, `blake2s_256`
- Blake3: `blake3`
- MD5: `md5`

## License

Licensed under the Apache License, Version 2.0. See `LICENSE`.
