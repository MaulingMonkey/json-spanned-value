/// Deserialization/parsing settings
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// Allow duplicate JSON object/map keys when deserializing [Map](crate::Map)s, such as: `{"a": 1, "a": 2}`.  Only one value will be retained.<br>
    /// **default: false**
    pub allow_duplicate_keys: bool,

    /// Allow trailing commas when deserializing an array such as `[1, 2, 3,]` or object such as `{"a", 1, "b": 2,}`.
    pub allow_trailing_comma: bool,

    /// Allow `// single line` or `/* block */` comments.
    pub allow_comments: bool,

    #[doc(hidden)] pub _non_exhaustive: ()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            allow_duplicate_keys:   false,
            allow_trailing_comma:   false,
            allow_comments:         false,

            _non_exhaustive:        ()
        }
    }
}
