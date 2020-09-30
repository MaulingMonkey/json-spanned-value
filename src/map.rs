use crate::{settings, Spanned};

use serde::de;

use std::borrow::Borrow;
use std::cmp::Ord;
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;



#[cfg(not(feature = "indexmap"))] type MapImpl<K, V> = std::collections::BTreeMap<K, V>;
#[cfg(    feature = "indexmap" )] type MapImpl<K, V> = indexmap::IndexMap<K, V>;

#[cfg(not(feature = "indexmap"))] type Entry<'s, K, V> = std::collections::btree_map::Entry<'s, K, V>;
#[cfg(    feature = "indexmap" )] type Entry<'s, K, V> = indexmap::map::Entry<'s, K, V>;

#[derive(Clone, Debug)]
/// A basic un-[Spanned] object/map, with [Spanned] children.
///
/// [Spanned]: super::Spanned
pub struct Map<K: Hash + Ord, V> {
    map: MapImpl<K, V>
}

impl<K: Hash + Ord, V> Map<K, V> {
    // https://docs.rs/indexmap/1.6.0/indexmap/map/struct.IndexMap.html
    // https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

    #[doc = "Makes a new empty Map."                                                        ] pub fn new() -> Self { Map { map: MapImpl::new() } }
    #[doc = "Returns the number of elements in the map."                                    ] pub fn len(&self) -> usize { self.map.len() }
    #[doc = "`true` if the map contains no elements."                                       ] pub fn is_empty(&self) -> bool { self.map.is_empty() }
    #[doc = "Clears the map, removing all elements."                                        ] pub fn clear(&mut self) { self.map.clear() }
    #[doc = "Inserts a key-value pair into the map, returning the replaced value, if any."  ] pub fn insert(&mut self, k: K, v: V) -> Option<V> { self.map.insert(k, v) }
    #[doc = "Gets the given key's corresponding entry in the map for in-place manipulation."] pub fn entry(&mut self, key: K) -> Entry<K, V> { self.map.entry(key) }

    #[doc = "`true` if the map contains the given key."                                     ] pub fn contains_key   <Q>(&self, key: &Q) -> bool                 where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.contains_key(key) }
    #[doc = "Return a reference to the value stored for key, if any."                       ] pub fn get            <Q>(&self, key: &Q) -> Option<&V>           where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.get(key) }
    #[doc = "Returns the key-value pair corresponding to the supplied key, if any."         ] pub fn get_key_value  <Q>(&self, key: &Q) -> Option<(&K, &V)>     where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.get_key_value(key) }
    #[doc = "Returns a mutable reference to the value stored for key, if any."              ] pub fn get_mut        <Q>(&mut self, key: &Q) -> Option<&mut V>   where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.get_mut(key) }
    #[doc = "Removes a key from the map, returning the previous value, if any."             ] pub fn remove         <Q>(&mut self, key: &Q) -> Option<V>        where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.remove(key) }
    #[doc = "Removes a key from the map, returning the previous key/value pair, if any."    ] pub fn remove_entry   <Q>(&mut self, key: &Q) -> Option<(K, V)>   where Q: Eq + Ord + Hash + ?Sized, K: Borrow<Q>{ self.map.remove_entry(key) }

    #[doc = "Gets an iterator over the keys of the map."            ] pub fn keys         (&self)     -> impl Iterator<Item = &K>             { self.map.keys().into_iter() }
    #[doc = "Gets an iterator over the values of the map."          ] pub fn values       (&self)     -> impl Iterator<Item = &V>             { self.map.values().into_iter() }
    #[doc = "Gets a mutable iterator over the values of the map."   ] pub fn values_mut   (&mut self) -> impl Iterator<Item = &mut V>         { self.map.values_mut().into_iter() }
    #[doc = "Gets an iterator over the entries of the map."         ] pub fn iter         (&self)     -> impl Iterator<Item = (&K, &V)>       { self.map.iter() }
    #[doc = "Gets a mutable iterator over the entries of the map."  ] pub fn iter_mut     (&mut self) -> impl Iterator<Item = (&K, &mut V)>   { self.map.iter_mut() }
}

impl<'a, K: Hash + Ord + 'a, V: 'a> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = <&'a MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&self.map).into_iter() }
}

impl<'a, K: Hash + Ord + 'a, V: 'a> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = <&'a mut MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&mut self.map).into_iter() }
}

impl<K: Hash + Ord, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = <MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.map.into_iter() }
}

impl<'a, K: Hash + Ord + 'a, V: 'a> IntoIterator for &'a Spanned<Map<K, V>> {
    type Item = (&'a K, &'a V);
    type IntoIter = <&'a MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&self.get_ref().map).into_iter() }
}

impl<'a, K: Hash + Ord + 'a, V: 'a> IntoIterator for &'a mut Spanned<Map<K, V>> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = <&'a mut MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { (&mut self.get_mut().map).into_iter() }
}

impl<K: Hash + Ord, V> IntoIterator for Spanned<Map<K, V>> {
    type Item = (K, V);
    type IntoIter = <MapImpl<K, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter { self.into_inner().map.into_iter() }
}

impl<'de, K: Debug + Hash + Ord + de::Deserialize<'de>, V: de::Deserialize<'de>> de::Deserialize<'de> for Map<K, V> {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MapVisitor<'de, K: Ord + de::Deserialize<'de>, V: de::Deserialize<'de>>(PhantomData<(&'de (), K, V)>);
        impl<'de, K: Debug + Hash + Ord + de::Deserialize<'de>, V: de::Deserialize<'de>> de::Visitor<'de> for MapVisitor<'de, K, V> {
            type Value = Map<K, V>;
            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result { formatter.write_str("a JSON object") }
            fn visit_map<MA: de::MapAccess<'de>>(self, mut visitor: MA) -> Result<Self::Value, MA::Error> {
                let mut values = Map::new();
                while let Some(key) = visitor.next_key()? {
                    let value = visitor.next_value()?;
                    if !settings().map_or(false, |s| s.allow_duplicate_keys) && values.contains_key(&key) {
                        return Err(de::Error::custom(format!("Duplicate field: {:?}", key)));
                    }
                    values.insert(key, value);
                }
                Ok(values)
            }
        }

        deserializer.deserialize_any(MapVisitor::<'de, K, V>(PhantomData))
    }
}
