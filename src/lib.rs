#![allow(dead_code)]

use serde_json::error::{Result as SJResult};

use serde::de;

use std::borrow::Borrow;
#[cfg_attr(feature = "indexmap", allow(unused_imports))] use std::collections::BTreeMap;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::cell::{Cell, RefCell};
use std::convert::*;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{self, Read};
use std::marker::PhantomData;
use std::ops::{Deref, Drop};
use std::rc::Rc;



#[derive(Clone)]
pub struct Spanned<V> {
    start:  usize,
    end:    usize,
    value:  V,
}

impl<V> Spanned<V> {
    pub fn start(&self) -> usize { self.start }
    pub fn end(&self) -> usize { self.end }
    pub fn span(&self) -> (usize, usize) { (self.start, self.end) }
    pub fn into_inner(self) -> V { self.value }
    pub fn get_ref(&self) -> &V { &self.value }
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

impl<'de, V: de::Deserialize<'de>> de::Deserialize<'de> for Spanned<V> {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (start, start_ch)   = start().unwrap_or((0, '\0'));
        let value               = V::deserialize(deserializer)?;
        let (end, _end_ch)      = end(!"[{ntf\"".contains(start_ch)).unwrap_or((0, '\0'));
        Ok(Self { start, end, value })
    }
}



pub enum Value {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<Spanned<Value>>),
    Object(Map<Spanned<String>, Spanned<Value>>),
}

impl Value {
    pub fn is_null      (&self) -> bool { match self { Value::Null      => true, _ => false } }
    pub fn is_bool      (&self) -> bool { match self { Value::Bool(_)   => true, _ => false } }
    pub fn is_number    (&self) -> bool { match self { Value::Number(_) => true, _ => false } }
    pub fn is_string    (&self) -> bool { match self { Value::String(_) => true, _ => false } }
    pub fn is_array     (&self) -> bool { match self { Value::Array(_)  => true, _ => false } }
    pub fn is_object    (&self) -> bool { match self { Value::Object(_) => true, _ => false } }

    pub fn as_null      (&self) -> Option<()>                                       { match self { Value::Null      => Some(()), _ => None } }
    pub fn as_bool      (&self) -> Option<bool>                                     { match self { Value::Bool(v)   => Some(*v), _ => None } }
    pub fn as_number    (&self) -> Option<&serde_json::Number>                      { match self { Value::Number(v) => Some(v), _ => None } }
    pub fn as_string    (&self) -> Option<&str>                                     { match self { Value::String(v) => Some(v), _ => None } }
    pub fn as_array     (&self) -> Option<&Vec<Spanned<Value>>>                     { match self { Value::Array(v)  => Some(v), _ => None } }
    pub fn as_object    (&self) -> Option<&Map<Spanned<String>, Spanned<Value>>>    { match self { Value::Object(v) => Some(v), _ => None } }

    // no as_null_mut
    pub fn as_bool_mut  (&mut self) -> Option<&mut bool>                                    { match self { Value::Bool(v) => Some(v), _ => None } }
    pub fn as_number_mut(&mut self) -> Option<&mut serde_json::Number>                      { match self { Value::Number(v) => Some(v), _ => None } }
    pub fn as_string_mut(&mut self) -> Option<&mut String>                                  { match self { Value::String(v) => Some(v), _ => None } }
    pub fn as_array_mut (&mut self) -> Option<&mut Vec<Spanned<Value>>>                     { match self { Value::Array(v) => Some(v), _ => None } }
    pub fn as_object_mut(&mut self) -> Option<&mut Map<Spanned<String>, Spanned<Value>>>    { match self { Value::Object(v) => Some(v), _ => None } }

    pub fn into_null    (self) -> Result<(), Self>                                      { match self { Value::Null      => Ok(()), o => Err(o) } }
    pub fn into_bool    (self) -> Result<bool, Self>                                    { match self { Value::Bool(v)   => Ok(v), o => Err(o) } }
    pub fn into_number  (self) -> Result<serde_json::Number, Self>                      { match self { Value::Number(v) => Ok(v), o => Err(o) } }
    pub fn into_string  (self) -> Result<String, Self>                                  { match self { Value::String(v) => Ok(v), o => Err(o) } }
    pub fn into_array   (self) -> Result<Vec<Spanned<Value>>, Self>                     { match self { Value::Array(v)  => Ok(v), o => Err(o) } }
    pub fn into_object  (self) -> Result<Map<Spanned<String>, Spanned<Value>>, Self>    { match self { Value::Object(v) => Ok(v), o => Err(o) } }
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



#[cfg(not(feature = "indexmap"))] type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(    feature = "indexmap" )] type MapImpl<K, V> = indexmap::IndexMap<K, V>;

pub struct Map<K: Hash + Ord, V> {
    map: MapImpl<K, V>
}

impl<K: Hash + Ord, V> Map<K, V> {
    pub fn new() -> Self { Map { map: MapImpl::new() } }
    pub fn clear(&mut self) { self.map.clear() }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> { self.map.insert(k, v) }
    pub fn len(&self) -> usize { self.map.len() }

    pub fn keys         (&self)     -> impl Iterator<Item = &K>             { self.map.keys().into_iter() }
    pub fn values       (&self)     -> impl Iterator<Item = &V>             { self.map.values().into_iter() }
    pub fn values_mut   (&mut self) -> impl Iterator<Item = &mut V>         { self.map.values_mut().into_iter() }
    pub fn iter         (&self)     -> impl Iterator<Item = (&K, &V)>       { self.map.iter() }
    pub fn iter_mut     (&mut self) -> impl Iterator<Item = (&K, &mut V)>   { self.map.iter_mut() }
}

impl<'de, K: Hash + Ord + de::Deserialize<'de>, V: de::Deserialize<'de>> de::Deserialize<'de> for Map<K, V> {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MapVisitor<'de, K: Ord + de::Deserialize<'de>, V: de::Deserialize<'de>>(PhantomData<(&'de (), K, V)>);
        impl<'de, K: Hash + Ord + de::Deserialize<'de>, V: de::Deserialize<'de>> de::Visitor<'de> for MapVisitor<'de, K, V> {
            type Value = Map<K, V>;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result { formatter.write_str("a JSON object") }
            fn visit_map<MA: de::MapAccess<'de>>(self, mut visitor: MA) -> Result<Self::Value, MA::Error> {
                let mut values = Map::new();
                while let Some(key) = visitor.next_key()? {
                    let value = visitor.next_value()?;
                    values.insert(key, value);
                }
                Ok(values)
            }
        }

        deserializer.deserialize_any(MapVisitor::<'de, K, V>(PhantomData))
    }
}



pub mod spanned {
    pub use super::Spanned;

    pub type Null       = Spanned<()>;
    pub type Bool       = Spanned<bool>;
    pub type Num<'n>    = Spanned<&'n serde_json::Number>;
    pub type Number     = Spanned<serde_json::Number>;
    pub type Str<'s>    = Spanned<&'s str>;
    pub type String     = Spanned<std::string::String>;
    pub type Span<'s>   = Spanned<&'s [self::Value]>;
    pub type Array      = Spanned<Vec<self::Value>>;
    pub type Obj<'o>    = Spanned<&'o super::Map<self::String, self::Value>>;
    pub type Object     = Spanned<super::Map<self::String, self::Value>>;
    pub type Value      = Spanned<super::Value>;

    impl Value {
        pub fn as_span_null      (&self) -> Option<Null>    { match  self.value { super::Value::Null      => Some(Spanned { start: self.start, end: self.end, value: () }), _ => None } }
        pub fn as_span_bool      (&self) -> Option<Bool>    { match  self.value { super::Value::Bool(v)   => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }
        pub fn as_span_number    (&self) -> Option<Num>     { match &self.value { super::Value::Number(v) => Some(Spanned { start: self.start, end: self.end, value: &v }), _ => None } }
        pub fn as_span_string    (&self) -> Option<Str>     { match &self.value { super::Value::String(v) => Some(Spanned { start: self.start, end: self.end, value: &v }), _ => None } }
        pub fn as_span_array     (&self) -> Option<Span>    { match &self.value { super::Value::Array(v)  => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }
        pub fn as_span_object    (&self) -> Option<Obj>     { match &self.value { super::Value::Object(v) => Some(Spanned { start: self.start, end: self.end, value: v  }), _ => None } }

        // TODO: as_span_*_mut ?  how would that even work?

        pub fn into_span_null    (self) -> Result<Null,     Self> { let Self { start, end, value } = self; match value { super::Value::Null      => Ok(Spanned { start, end, value: () }), value => Err(Spanned { start, end, value }) } }
        pub fn into_span_bool    (self) -> Result<Bool,     Self> { let Self { start, end, value } = self; match value { super::Value::Bool(v)   => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
        pub fn into_span_number  (self) -> Result<Number,   Self> { let Self { start, end, value } = self; match value { super::Value::Number(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
        pub fn into_span_string  (self) -> Result<String,   Self> { let Self { start, end, value } = self; match value { super::Value::String(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
        pub fn into_span_array   (self) -> Result<Array,    Self> { let Self { start, end, value } = self; match value { super::Value::Array(v)  => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
        pub fn into_span_object  (self) -> Result<Object,   Self> { let Self { start, end, value } = self; match value { super::Value::Object(v) => Ok(Spanned { start, end, value: v  }), value => Err(Spanned { start, end, value }) } }
    }
}



#[derive(Default)]
struct Shared {
    last_read:  Cell<u8>,
    start:      Cell<(usize, u8)>,
    prev:       Cell<(usize, u8)>,
    pos:        Cell<(usize, u8)>,
}
thread_local! { static SHARED : RefCell<Option<Rc<Shared>>> = RefCell::new(None); }
fn last_read()      -> Option<u8>    { SHARED.with(|s| s.borrow().as_ref().map(|s| s.last_read.get())) }
fn start()          -> Option<(usize, char)> { SHARED.with(|s| s.borrow().as_ref().map(|s| s.start.get())).map(|(s,c)| (s, c as char)) }
fn end(prev: bool)  -> Option<(usize, char)> { SHARED.with(|s| s.borrow().as_ref().map(|s| if prev { s.prev.get() } else { s.pos.get() })).map(|(s,c)| (s, c as char)) }

struct SharedStack(Option<Rc<Shared>>);

impl SharedStack {
    pub fn push(shared: Rc<Shared>) -> Self {
        SHARED.with(|s| {
            let mut s = s.borrow_mut();
            Self(std::mem::replace(&mut *s, Some(shared)))
        })
    }
}

impl Drop for SharedStack {
    fn drop(&mut self) {
        SHARED.with(|s| std::mem::swap(&mut *s.borrow_mut(), &mut self.0));
    }
}



struct Reader<B: Buffer> {
    buf:    B,
    shared: Rc<Shared>,
}

impl<B: Buffer> Read for Reader<B> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if out.is_empty() { return Ok(0) }

        let shared = &*self.shared;
        let src = self.buf.as_bytes();
        let pos1 = shared.pos.get().0;
        let next = if let Some(n) = src.get(pos1) { *n } else { return Ok(0) };
        let pos2 = pos1 + 1;

        shared.last_read.set(next);
        shared.prev.set(shared.pos.get());
        shared.pos.set((pos2, next));
        out[0] = next;

        let start = if next == b'\"' {
            pos1
        } else {
            let mut start = pos1;
            while b": \r\n\t".contains(src.get(start).unwrap_or(&b'\0')) { start += 1; }
            start
        };
        shared.start.set((start, src[start]));

        Ok(1)
    }
}

impl<T: AsRef<[u8]>> Buffer for T {}
trait Buffer : AsRef<[u8]> {
    fn as_bytes(&self) -> &[u8] { self.as_ref() }
    fn len(&self) -> usize { self.as_bytes().len() }
}



pub fn from_slice<T: de::DeserializeOwned>(buf: &[u8]) -> SJResult<T> {
    let shared = Rc::new(Shared::default());
    let _shared_stack = SharedStack::push(shared.clone());
    // NOTE:  Our use of from_reader forces us to use DeserializeOwned
    serde_json::from_reader(Reader {
        buf,
        shared,
    })
}

pub fn from_str<T: de::DeserializeOwned>(buf: &str) -> SJResult<T> {
    from_slice(buf.as_bytes())
}



#[cfg(test)] mod tests {
    use super::*;
    use serde::Deserialize;

    fn do_test_obj(json: &str, expected: Vec<(&str, &str, fn(&Value) -> bool)>) {
        let expected = expected.into_iter().map(|(k, v, v2)| (k, (v, v2))).collect::<BTreeMap<_, _>>();
        let parsed : Value = super::from_str(json).unwrap();
        for (key, val) in parsed.into_object().unwrap().iter() {
            let key_raw = json.get(key.start .. key.end).unwrap_or_else(|| panic!("Unable to fetch key_raw for key: {:?}", key));
            let val_raw = json.get(val.start .. val.end).unwrap_or_else(|| panic!("Unable to fetch val_raw for val: {:?}", val));
            let (expected_raw_val, expected_val) = expected.get(key.as_str()).unwrap_or_else(|| panic!("Did not expect key: {:?}", key));

            assert_eq!(key_raw, format!("{:?}", key));
            assert_eq!(val_raw, *expected_raw_val);
            assert!(expected_val(val), "Failed to match condition for val: {:?}", val);
        }
    }

    #[test] fn object_no_whitespace() {
        do_test_obj("{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}", vec![
            ("null",    "null",                                     |v| v.is_null()),
            ("string",  "\"string\\nwith\\nescapes\"",              |v| v.as_string() == Some("string\nwith\nescapes")),
            ("number",  "123",                                      |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
            ("array",   "[1,2,3]",                                  |v| v.as_array().map(|a| a.len()) == Some(3)),
            ("object",  "{\"key\":\"value\",\"key2\":\"value2\"}",  |v| v.as_object().map(|o| o.len()) == Some(2)),
        ]);
    }

    #[test] fn object_more_whitespace() {
        do_test_obj("{  \"null\"  :  null  ,  \"string\"  :  \"string\\nwith\\nescapes\"  ,  \"number\"  :  123  ,  \"array\"  :  [1,2,3]  ,  \"object\"  :  {\"key\":\"value\",\"key2\":\"value2\"}}", vec![
            ("null",    "null",                                     |v| v.is_null()),
            ("string",  "\"string\\nwith\\nescapes\"",              |v| v.as_string() == Some("string\nwith\nescapes")),
            ("number",  "123",                                      |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
            ("array",   "[1,2,3]",                                  |v| v.as_array().map(|a| a.len()) == Some(3)),
            ("object",  "{\"key\":\"value\",\"key2\":\"value2\"}",  |v| v.is_object()),
        ]);
    }

    #[test] fn object_even_more_whitespace() {
        do_test_obj("{  \"  null  \"  :  null  ,  \"  string  \"  :  \"  string\\nwith\\nescapes  \"  ,  \"  number  \"  :  123  ,  \"  array  \"  :  [  1  ,  2  ,  3  ]  ,  \"  object  \"  :  {  \"  key  \"  :  \"  value  \"  ,  \"  key2  \"  :  \"  value2  \"  }  }", vec![
            ("  null  ",    "null",                                 |v| v.is_null()),
            ("  string  ",  "\"  string\\nwith\\nescapes  \"",      |v| v.as_string() == Some("  string\nwith\nescapes  ")),
            ("  number  ",  "123",                                  |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
            ("  array  ",   "[  1  ,  2  ,  3  ]",                  |v| v.as_array().map(|a| a.len()) == Some(3)),
            ("  object  ",  "{  \"  key  \"  :  \"  value  \"  ,  \"  key2  \"  :  \"  value2  \"  }",  |v| v.as_object().map(|o| o.len()) == Some(2)),
        ]);
    }

    #[test] fn struct_plain() {
        #[derive(Deserialize)] struct Plain {
            null:   (),
            string: String,
            number: serde_json::Number,
            array:  Vec<Value>,
            object: Map<Spanned<String>, spanned::Value>,
        }

        let json = "{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}";
        let _parsed1 : Plain            = super::from_str(json).unwrap();
        let _parsed2 : Spanned<Plain>   = super::from_str(json).unwrap();
    }

    #[test] fn struct_annotated() {
        #[derive(Deserialize)] struct Annotated {
            null:   spanned::Null,
            string: spanned::String,
            number: spanned::Number,
            array:  spanned::Array,
            object: spanned::Object,
        }

        let json = "{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}";
        let _parsed1 : Annotated            = super::from_str(json).unwrap();
        let _parsed2 : Spanned<Annotated>   = super::from_str(json).unwrap();
    }
}
