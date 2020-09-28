use serde::de;

#[cfg_attr(feature = "indexmap", allow(unused_imports))] use std::collections::BTreeMap;
use std::cmp::Ord;
use std::fmt::{self, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;



#[cfg(not(feature = "indexmap"))] type MapImpl<K, V> = BTreeMap<K, V>;
#[cfg(    feature = "indexmap" )] type MapImpl<K, V> = indexmap::IndexMap<K, V>;

#[derive(Clone, Debug)]
/// A basic un-[Spanned] object/map, with [Spanned] children.
///
/// [Spanned]: super::Spanned
pub struct Map<K: Hash + Ord, V> {
    map: MapImpl<K, V>
}

impl<K: Hash + Ord, V> Map<K, V> {
    #[doc = "Makes a new empty Map."                                                        ] pub fn new() -> Self { Map { map: MapImpl::new() } }
    #[doc = "Clears the map, removing all elements."                                        ] pub fn clear(&mut self) { self.map.clear() }
    #[doc = "Inserts a key-value pair into the map, returning the replaced value (if any.)" ] pub fn insert(&mut self, k: K, v: V) -> Option<V> { self.map.insert(k, v) }
    #[doc = "Returns the number of elements in the map."                                    ] pub fn len(&self) -> usize { self.map.len() }

    #[doc = "Gets an iterator over the keys of the map."            ] pub fn keys         (&self)     -> impl Iterator<Item = &K>             { self.map.keys().into_iter() }
    #[doc = "Gets an iterator over the values of the map."          ] pub fn values       (&self)     -> impl Iterator<Item = &V>             { self.map.values().into_iter() }
    #[doc = "Gets a mutable iterator over the values of the map."   ] pub fn values_mut   (&mut self) -> impl Iterator<Item = &mut V>         { self.map.values_mut().into_iter() }
    #[doc = "Gets an iterator over the entries of the map."         ] pub fn iter         (&self)     -> impl Iterator<Item = (&K, &V)>       { self.map.iter() }
    #[doc = "Gets a mutable iterator over the entries of the map."  ] pub fn iter_mut     (&mut self) -> impl Iterator<Item = (&K, &mut V)>   { self.map.iter_mut() }
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
