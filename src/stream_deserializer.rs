// https://docs.rs/serde_json/1.0.58/serde_json/de/struct.StreamDeserializer.html

use crate::{Reader, Settings, Shared, SharedStack};

use serde::Deserialize;

use serde_json::de::{IoRead, StreamDeserializer as BaseStreamDeserializer};
use serde_json::{Result};

use std::iter::{FusedIterator, Iterator};
use std::sync::Arc;



/// Iterator that deserializes a stream into multiple JSON values.
pub struct StreamDeserializer<'de, B: AsRef<[u8]>, T: Deserialize<'de>> {
    base:   BaseStreamDeserializer<'de, IoRead<Reader<B>>, T>,
    shared: Arc<Shared>,
}

impl<'de, B: AsRef<[u8]>, T: Deserialize<'de>> StreamDeserializer<'de, B, T> {
    /// Create a JSON stream deserializer
    pub fn new(buffer: B) -> Self { Self::new_with_settings(buffer, Settings::default()) }

    /// Create a JSON stream deserializer, with settings
    pub fn new_with_settings(buffer: B, settings: Settings) -> Self {
        let shared = Arc::new(Shared::new(&settings));
        let _shared_stack = SharedStack::push(shared.clone());
        Self {
            base:   BaseStreamDeserializer::new(IoRead::new(Reader::new(buffer, shared.clone()))),
            shared,
        }
    }

    /// Returns the number of bytes so far deserialized into a successful `T`.
    pub fn byte_offset(&self) -> usize { self.base.byte_offset() }
}

impl<'de, B: AsRef<[u8]>, T: Deserialize<'de>> Iterator for StreamDeserializer<'de, B, T> {
    type Item = Result<T>;
    fn next(&mut self) -> Option<Result<T>> {
        let _shared_stack = SharedStack::push(self.shared.clone());
        self.base.next()
    }
}

// https://github.com/serde-rs/json/blob/v1.0.58/src/read.rs#L769-L772
impl<'de, B: AsRef<[u8]>, T: Deserialize<'de>> FusedIterator for StreamDeserializer<'de, B, T> {}
