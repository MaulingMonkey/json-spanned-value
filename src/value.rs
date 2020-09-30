use crate::*;

use serde::de;

use std::convert::*;
use std::fmt::{self, Debug, Formatter};



/// A basic un-[Spanned] value, with [Spanned] children.
/// Unless you want to `match` it, you probably want [spanned::Value].
pub enum Value {
    #[doc="`null`"                                 ] Null,
    #[doc="`true` or `false`"                      ] Bool(bool),
    #[doc="A number like `123`"                    ] Number(serde_json::Number),
    #[doc="A string like `\"asdf\"`"               ] String(String),
    #[doc="An array like `[1, 2, 3]`"              ] Array(Vec<Spanned<Value>>),
    #[doc="An object like `{\"a\": 1, \"b\": 2}`"  ] Object(Map<Spanned<String>, Spanned<Value>>),
}

impl Value {
    #[doc="True if self is `null`"                                          ] pub fn is_null    (&self) -> bool { match self { Value::Null      => true, _ => false } }
    #[doc="True if self is `true` or `false`"                               ] pub fn is_bool    (&self) -> bool { match self { Value::Bool(_)   => true, _ => false } }
    #[doc="True if self is a number like `123`"                             ] pub fn is_number  (&self) -> bool { match self { Value::Number(_) => true, _ => false } }
    #[doc="True if self is a string like `\"asdf\"`"                        ] pub fn is_string  (&self) -> bool { match self { Value::String(_) => true, _ => false } }
    #[doc="True if self is an array like `[1, 2, 3]`"                       ] pub fn is_array   (&self) -> bool { match self { Value::Array(_)  => true, _ => false } }
    #[doc="True if self is an object like `{\"a\": 1, \"b\": 2}`"           ] pub fn is_object  (&self) -> bool { match self { Value::Object(_) => true, _ => false } }
 
    #[doc="`Some(()) if self is `null`"                                     ] pub fn as_null      (&self) -> Option<()>                                       { match self { Value::Null      => Some(()), _ => None } }
    #[doc="`Some(inner)` if self is `true` or `false`"                      ] pub fn as_bool      (&self) -> Option<bool>                                     { match self { Value::Bool(v)   => Some(*v), _ => None } }
    #[doc="`Some(&inner)` if self is a number like `123`"                   ] pub fn as_number    (&self) -> Option<&serde_json::Number>                      { match self { Value::Number(v) => Some(v), _ => None } }
    #[doc="`Some(&inner)` if self is a string like `\"asdf\"`"              ] pub fn as_string    (&self) -> Option<&str>                                     { match self { Value::String(v) => Some(v), _ => None } }
    #[doc="`Some(&inner)` if self is an array like `[1, 2, 3]`"             ] pub fn as_array     (&self) -> Option<&Vec<Spanned<Value>>>                     { match self { Value::Array(v)  => Some(v), _ => None } }
    #[doc="`Some(&inner)` if self is an object like `{\"a\": 1, \"b\": 2}`" ] pub fn as_object    (&self) -> Option<&Map<Spanned<String>, Spanned<Value>>>    { match self { Value::Object(v) => Some(v), _ => None } }

    // no as_null_mut
    #[doc="`Some(&mut v)` if self is `true` or `false`"                     ] pub fn as_bool_mut  (&mut self) -> Option<&mut bool>                                    { match self { Value::Bool(v) => Some(v), _ => None } }
    #[doc="`Some(&mut v)` if self is a number like `123`"                   ] pub fn as_number_mut(&mut self) -> Option<&mut serde_json::Number>                      { match self { Value::Number(v) => Some(v), _ => None } }
    #[doc="`Some(&mut v)` if self is a string like `\"asdf\"`"              ] pub fn as_string_mut(&mut self) -> Option<&mut String>                                  { match self { Value::String(v) => Some(v), _ => None } }
    #[doc="`Some(&mut v)` if self is an array like `[1, 2, 3]`"             ] pub fn as_array_mut (&mut self) -> Option<&mut Vec<Spanned<Value>>>                     { match self { Value::Array(v) => Some(v), _ => None } }
    #[doc="`Some(&mut v)` if self is an object like `{\"a\": 1, \"b\": 2}`" ] pub fn as_object_mut(&mut self) -> Option<&mut Map<Spanned<String>, Spanned<Value>>>    { match self { Value::Object(v) => Some(v), _ => None } }

    #[doc="`Ok(inner)` if self is `null`, otherwise `Err(self)`"                                ] pub fn into_null    (self) -> Result<(), Self>                                      { match self { Value::Null      => Ok(()), o => Err(o) } }
    #[doc="`Ok(inner)` if self is `true` or `false`, otherwise `Err(self)`"                     ] pub fn into_bool    (self) -> Result<bool, Self>                                    { match self { Value::Bool(v)   => Ok(v), o => Err(o) } }
    #[doc="`Ok(inner)` if self is a number like `123`, otherwise `Err(self)`"                   ] pub fn into_number  (self) -> Result<serde_json::Number, Self>                      { match self { Value::Number(v) => Ok(v), o => Err(o) } }
    #[doc="`Ok(inner)` if self is a string like `\"asdf\"`, otherwise `Err(self)`"              ] pub fn into_string  (self) -> Result<String, Self>                                  { match self { Value::String(v) => Ok(v), o => Err(o) } }
    #[doc="`Ok(inner)` if self is an array like `[1, 2, 3]`, otherwise `Err(self)`"             ] pub fn into_array   (self) -> Result<Vec<Spanned<Value>>, Self>                     { match self { Value::Array(v)  => Ok(v), o => Err(o) } }
    #[doc="`Ok(inner)` if self is an object like `{\"a\": 1, \"b\": 2}`, otherwise `Err(self)`" ] pub fn into_object  (self) -> Result<Map<Spanned<String>, Spanned<Value>>, Self>    { match self { Value::Object(v) => Ok(v), o => Err(o) } }

    /// A human-readable representation of the type of this value
    pub fn type_str(&self) -> &'static str {
        match self {
            Value::Null         => "null",
            Value::Bool(_)      => "boolean",
            Value::Number(_)    => "number",
            Value::String(_)    => "string",
            Value::Array(_)     => "array",
            Value::Object(_)    => "object",
        }
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Value::Null         => write!(fmt, "null"),
            Value::Bool(true)   => write!(fmt, "true"),
            Value::Bool(false)  => write!(fmt, "false"),
            Value::Number(n)    => write!(fmt, "{}", n),
            Value::String(s)    => write!(fmt, "{:?}", s),
            Value::Array(a) => {
                write!(fmt, "[")?;
                if a.len() > 32 {
                    write!(fmt, "...")?;
                } else {
                    let mut first = true;
                    for e in a.iter() {
                        write!(fmt, "{}", if first { " " } else { ", " })?;
                        write!(fmt, "{:?}", e)?;
                        first = false;
                    }
                    if !first { write!(fmt, " ")?; }
                }
                write!(fmt, "]")
            },
            Value::Object(o) => {
                write!(fmt, "{{")?;
                if o.len() > 32 {
                    write!(fmt, "...")?;
                } else {
                    let mut first = true;
                    for (k, v) in o.iter() {
                        write!(fmt, "{}", if first { " " } else { ", " })?;
                        write!(fmt, "{:?}: {:?}", k, v)?;
                        first = false;
                    }
                    if !first { write!(fmt, " ")?; }
                }
                write!(fmt, "}}")
            },
        }
    }
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ValueVisitor;
        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result { formatter.write_str("any valid JSON value") }

            fn visit_bool   <E>(self, value: bool)      -> Result<Value, E> { Ok(Value::Bool(value)) }
            fn visit_i64    <E>(self, value: i64)       -> Result<Value, E> { Ok(Value::Number(value.into())) }
            fn visit_u64    <E>(self, value: u64)       -> Result<Value, E> { Ok(Value::Number(value.into())) }
            fn visit_f64    <E>(self, value: f64)       -> Result<Value, E> { Ok(serde_json::Number::from_f64(value).map_or(Value::Null, Value::Number)) }
            fn visit_str    <E>(self, value: &str)      -> Result<Value, E> { Ok(Value::String(String::from(value))) }
            fn visit_string <E>(self, value: String)    -> Result<Value, E> { Ok(Value::String(value)) }
            fn visit_none   <E>(self)                   -> Result<Value, E> { Ok(Value::Null) }
            fn visit_unit   <E>(self)                   -> Result<Value, E> { Ok(Value::Null) }

            fn visit_some<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> { de::Deserialize::deserialize(deserializer) }

            fn visit_seq<V: de::SeqAccess<'de>>(self, mut visitor: V) -> Result<Value, V::Error> {
                let mut vec = Vec::new();
                while let Some(elem) = visitor.next_element()? { vec.push(elem); }
                Ok(Value::Array(vec))
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut visitor: V) -> Result<Value, V::Error> {
                let mut values = Map::new();
                while let Some(key) = visitor.next_key()? {
                    let value = visitor.next_value()?;
                    values.insert(key, value);
                }
                Ok(Value::Object(values))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
