/// Deserialization/parsing settings
#[derive(Clone, Copy, Debug)]
pub struct Settings {
    /// Allow duplicate JSON object/map keys when deserializing [Map](crate::Map)s, such as: `{"a": 1, "a": 2}`.  Only one value will be retained.<br>
    /// **default: false**
    pub allow_duplicate_keys: bool,

    #[doc(hidden)] pub _non_exhaustive: ()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            allow_duplicate_keys:   false,

            _non_exhaustive:        ()
        }
    }
}
