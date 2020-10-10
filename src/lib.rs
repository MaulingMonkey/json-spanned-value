//! Track the origin of your json values for better error reporting!
//! The [toml] crate has [toml-spanned-value] for this.
//! [serde_json] now has [json-spanned-value].
//! 
//! The basic crates provide users with a `Value` type that can be used for custom parsing logic.
//! However, this type doesn't support span information.
//! In some cases it's possible to extract line/column information out of error messages,
//! but that's awkward and error prone - often reporting errors on the next line
//! (e.g. where the seek position of the underlying reader has skipped to.)
//!
//!
//!
//! [serde_json]:           https://docs.rs/serde_json/
//! [toml]:                 https://docs.rs/toml/
//! [toml-spanned-value]:   https://docs.rs/toml-spanned-value/
//! [json-spanned-value]:   https://docs.rs/json-spanned-value/
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

mod error_ext;              pub use error_ext::*;
mod map;                    pub use map::Map;
mod reader;                 pub(crate) use reader::*;
mod settings;               pub use settings::*;
mod shared;                 pub(crate) use shared::*;
pub mod spanned;            pub use spanned::Spanned;
mod stream_deserializer;    pub use stream_deserializer::StreamDeserializer;
mod value;                  pub use value::Value;

#[cfg(test)] mod tests;



use serde_json::error as sje;
use serde::de;
use std::rc::Rc;


/// Read json from a slice of in-memory bytes, with explicit [Settings]
pub fn from_slice_with_settings<T: de::DeserializeOwned>(buf: &[u8], settings: &Settings) -> sje::Result<T> {
    let shared = Rc::new(Shared::new(settings));
    let _shared_stack = SharedStack::push(shared.clone());
    // NOTE:  Our use of from_reader forces us to use DeserializeOwned
    serde_json::from_reader(Reader::new(buf, shared))
}

/// Read json from a slice of in-memory bytes, with default [Settings]
pub fn from_slice<T: de::DeserializeOwned>(buf: &[u8]) -> sje::Result<T> {
    from_slice_with_settings(buf, &Settings::default())
}

/// Read json from an in-memory string, with explicit [Settings]
pub fn from_str_with_settings<T: de::DeserializeOwned>(buf: &str, settings: &Settings) -> sje::Result<T> {
    from_slice_with_settings(buf.as_bytes(), settings)
}

/// Read json from an in-memory string, with default [Settings]
pub fn from_str<T: de::DeserializeOwned>(buf: &str) -> sje::Result<T> {
    from_slice(buf.as_bytes())
}
