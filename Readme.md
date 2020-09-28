# json-spanned-value

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/json-spanned-value.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/json-spanned-value)
[![crates.io](https://img.shields.io/crates/v/json-spanned-value.svg)](https://crates.io/crates/json-spanned-value)
[![docs.rs](https://docs.rs/json-spanned-value/badge.svg)](https://docs.rs/json-spanned-value)
[![%23![forbid(unsafe_code)]](https://img.shields.io/github/search/MaulingMonkey/json-spanned-value/unsafe%2bextension%3Ars?color=green&label=%23![forbid(unsafe_code)])](https://github.com/MaulingMonkey/json-spanned-value/search?q=forbid%28unsafe_code%29+extension%3Ars)
[![rust: 1.46.0](https://img.shields.io/badge/rust-1.46.0%2B-yellow.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/json-spanned-value.svg)](https://github.com/MaulingMonkey/json-spanned-value)
[![Build Status](https://travis-ci.com/MaulingMonkey/json-spanned-value.svg?branch=master)](https://travis-ci.com/MaulingMonkey/json-spanned-value)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/json-spanned-value/status.svg)](https://deps.rs/repo/github/MaulingMonkey/json-spanned-value) -->


Track the origin of your json values for better error reporting!
The [toml] crate has [toml-spanned-value] for this.
[serde_json] now has [json-spanned-value].

The basic crates provide users with a `Value` type that can be used for custom parsing logic.
However, this type doesn't support span information.
In some cases it's possible to extract line/column information out of error messages,
but that's awkward and error prone - often reporting errors on the next line
(e.g. where the seek positino of the underlying reader has skipped to.)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[serde_json]:           https://docs.rs/serde_json/
[toml]:                 https://docs.rs/toml/
[toml-spanned-value]:   https://docs.rs/toml-spanned-value/
[json-spanned-value]:   https://docs.rs/json-spanned-value/
