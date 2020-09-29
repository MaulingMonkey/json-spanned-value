//! [Spanned] and aliases thereof -
//! spanned::{[Value], [Null], [Bool],
//! [Num]\[[ber](Number)\],
//! [Str]\[[ing](String)\],
//! [Obj]\[[ect](Object)\],
//! [Span], [Array]}

use super::{start, end};

use serde::de::*;

use std::borrow::Borrow;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::convert::*;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, Range};



/// A value with start/end position information.
/// Can wrap arbitrary [Deserialize]able JSON values, not just basic JSON types.
#[derive(Clone)]
pub struct Spanned<V> {
    pub(crate) start:  usize,
    pub(crate) end:    usize,
    pub(crate) value:  V,
}

impl<V> Spanned<V> {
    /// Get the starting byte offset (inclusive) of this value.
    /// Likely `0` unless loaded through [crate::from_*](crate::from_slice).
    pub fn start(&self) -> usize { self.start }

    /// Get the ending byte offset (non-inclusive) of this value.
    /// Likely `0` unless loaded through [crate::from_*](crate::from_slice).
    pub fn end(&self) -> usize { self.end }

    /// Get the start .. end byte offset of this value as a (start, end) tuple.
    /// Likely `(0, 0)` unless loaded through [crate::from_*](crate::from_slice).
    pub fn span(&self) -> (usize, usize) { (self.start, self.end) }

    /// Get the start .. end byte offset of this value as a start .. end [Range].
    /// Likely `0 .. 0` unless loaded through [crate::from_*](crate::from_slice).
    pub fn range(&self) -> Range<usize> { self.start .. self.end }

    /// Get the interior value of the spanned region as an owned value.
    pub fn into_inner(self) -> V { self.value }

    /// Get the interior value of the spanned region as a reference.
    pub fn get_ref(&self) -> &V { &self.value }

    /// Get the interior value of the spanned region as a mutable/exclusive reference.
    pub fn get_mut(&mut self) -> &mut V { &mut self.value }
}

impl                    Borrow<str> for Spanned<String> { fn borrow(&self) -> &str { self.get_ref() } }
impl<V>                 Deref       for Spanned<V> { fn deref(&self) -> &Self::Target { &self.value } type Target = V; }
//impl<V>               DerefMut    for Spanned<V> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value } }
impl<R, V: AsRef<R>>    AsRef<R>    for Spanned<V> { fn as_ref(&self) -> &R { self.value.as_ref() } }
impl<V: Debug>          Debug       for Spanned<V> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.value.fmt(fmt) } }
impl<V>                 From<V>     for Spanned<V> { fn from(v: V) -> Self { Self { value: v, start: 0, end: 0 } } }
impl<V: Eq>             Eq          for Spanned<V> {}
impl<V: Ord>            Ord         for Spanned<V> { fn cmp(&self, other: &Self) -> Ordering { self.value.cmp(&other.value) } }
impl<V: PartialEq>      PartialEq   for Spanned<V> { fn eq(&self, other: &Self) -> bool { self.value.eq(&other.value) } }
impl<V: PartialOrd>     PartialOrd  for Spanned<V> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.value.partial_cmp(&other.value) } }
impl<V: Hash>           Hash        for Spanned<V> { fn hash<H: Hasher>(&self, hasher: &mut H) { self.value.hash(hasher) } }

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Spanned<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (start, start_ch)   = start().unwrap_or((0, '\0'));
        let value               = V::deserialize(deserializer)?;
        let end                 = end(!"[{ntf\"".contains(start_ch)).unwrap_or(0);
        Ok(Self { start, end, value })
    }
}



#[doc = "Owned, arbitrary json value + span information"                ] pub type Value    = Spanned<super::Value>;
#[doc = "`null` + span information"                                     ] pub type Null     = Spanned<()>;
#[doc = "`true` or `false` + span information"                          ] pub type Bool     = Spanned<bool>;
#[doc = "Borrowed number like `123` + span information"                 ] pub type Num<'n>  = Spanned<&'n serde_json::Number>;
#[doc = "Owned number like `123` + span information"                    ] pub type Number   = Spanned<serde_json::Number>;
#[doc = "Borrowed string like `\"abc\"` + span information"             ] pub type Str<'s>  = Spanned<&'s str>;
#[doc = "Owned string like `\"abc\"` + span information"                ] pub type String   = Spanned<std::string::String>;
#[doc = "Borrowed object like `{\"a\":1, \"b\":2}` + span information"  ] pub type Obj<'o>  = Spanned<&'o super::Map<self::String, self::Value>>;
#[doc = "Owned object like `{\"a\":1, \"b\":2}` + span information"     ] pub type Object   = Spanned<super::Map<self::String, self::Value>>;
#[doc = "Borrowed array like `[1,2,3]` + span information"              ] pub type Span<'s> = Spanned<&'s [self::Value]>;
#[doc = "Owned array like `[1,2,3]` + span information"                 ] pub type Array    = Spanned<Vec<self::Value>>;

/// Various conversion methods:
///
/// * `as_span_[type]()` returns borrowing `Option`s of some sort.
/// * `into_span_[type]()` returns `Ok(type)` or `Err(original)`.
///
/// See also on [super::Value] as provided by `Deref` implementation:
///
/// * `is_[type]()` returns bools with obvious meanings.
/// * `as_[type]()` for borrowing `Option`s without span info.
/// * `into_[type]()` for `Ok(type)` without span info, or `Err(original)`.
impl Value {
    #[doc="`Some(span + ()) if self is `null`"                                     ] pub fn as_span_null    (&self) -> Option<Null>    { match  self.value { super::Value::Null      => Some(Spanned { start: self.start, end: self.end, value: () }), _ => None } }
    #[doc="`Some(span + inner)` if self is `true` or `false`"                      ] pub fn as_span_bool    (&self) -> Option<Bool>    { match  self.value { super::Value::Bool(v)   => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }
    #[doc="`Some(span + &inner)` if self is a number like `123`"                   ] pub fn as_span_number  (&self) -> Option<Num>     { match &self.value { super::Value::Number(v) => Some(Spanned { start: self.start, end: self.end, value: &v }), _ => None } }
    #[doc="`Some(span + &inner)` if self is a string like `\"asdf\"`"              ] pub fn as_span_string  (&self) -> Option<Str>     { match &self.value { super::Value::String(v) => Some(Spanned { start: self.start, end: self.end, value: &v }), _ => None } }
    #[doc="`Some(span + &inner)` if self is an array like `[1, 2, 3]`"             ] pub fn as_span_array   (&self) -> Option<Span>    { match &self.value { super::Value::Array(v)  => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }
    #[doc="`Some(span + &inner)` if self is an object like `{\"a\": 1, \"b\": 2}`" ] pub fn as_span_object  (&self) -> Option<Obj>     { match &self.value { super::Value::Object(v) => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }

    // TODO: as_span_*_mut ?  how would that even work?

    #[doc="`Ok(span + ()) if self is `null`, otherwise Err(self)"                                       ] pub fn into_span_null     (self) -> Result<Null,     Self> { let Self { start, end, value } = self; match value { super::Value::Null      => Ok(Spanned { start, end, value: () }), value => Err(Spanned { start, end, value }) } }
    #[doc="`Ok(span + inner)` if self is `true` or `false`, otherwise Err(self)"                        ] pub fn into_span_bool     (self) -> Result<Bool,     Self> { let Self { start, end, value } = self; match value { super::Value::Bool(v)   => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
    #[doc="`Ok(span + inner)` if self is a number like `123`, otherwise Err(self)"                      ] pub fn into_span_number   (self) -> Result<Number,   Self> { let Self { start, end, value } = self; match value { super::Value::Number(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
    #[doc="`Ok(span + inner)` if self is a string like `\"asdf\"`, otherwise Err(self)"                 ] pub fn into_span_string   (self) -> Result<String,   Self> { let Self { start, end, value } = self; match value { super::Value::String(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
    #[doc="`Ok(span + inner)` if self is an array like `[1, 2, 3]`, otherwise Err(self)"                ] pub fn into_span_array    (self) -> Result<Array,    Self> { let Self { start, end, value } = self; match value { super::Value::Array(v)  => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
    #[doc="`Ok(span + inner)` if self is an object like `{\"a\": 1, \"b\": 2}`, otherwise Err(self)"    ] pub fn into_span_object   (self) -> Result<Object,   Self> { let Self { start, end, value } = self; match value { super::Value::Object(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
}
