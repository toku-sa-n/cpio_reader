# `cpio_reader`

[![GitHub Actions](https://github.com/toku-sa-n/cpio_reader/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/cpio_reader/actions)
[![Crates.io](https://img.shields.io/crates/v/cpio_reader)](https://crates.io/crates/cpio_reader)
![Crates.io](https://img.shields.io/crates/l/cpio_reader)
[![docs.rs](https://docs.rs/cpio_reader/badge.svg)](https://docs.rs/cpio_reader)

A library to read the contents of the cpio file. (.cpio)

This library is based on the design written `man 5 cpio` and supports these four formats.
- Old Binary Format
- Portable ASCII Format
- New ASCII Format
- New CRC Format

This library is `#![no_std]` compatible.

## Examples

```rust
use std::fs;

let cpio = fs::read("tests/newc.cpio").unwrap();

for entry in cpio_reader::iter_files(&cpio) {
    println!("Entry name: {}, content: {:?}", entry.name(), entry.file());
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
