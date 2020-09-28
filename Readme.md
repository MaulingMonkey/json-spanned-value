# json-spanned-value

The [toml] crate has [toml-spanned-value].  The [serde_json] crate now has [json-spanned-value].

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
