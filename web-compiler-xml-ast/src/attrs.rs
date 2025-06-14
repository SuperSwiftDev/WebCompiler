use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Display};
use std::hash::Hasher;
use std::{cmp::Ordering, hash::Hash};
use std::iter::IntoIterator;
use std::ops::{Deref, Index, IndexMut};
use indexmap::IndexMap;

// NOTE: keep indexmap dependencies internal (so it can be swapped if necessary) — prefer newtypes.

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE KEYS — BORROWED API
// ————————————————————————————————————————————————————————————————————————————

/// Borrowed attribute key, wrapper over `str` for type safety
#[repr(transparent)]
pub struct AttributeKeyStr(str);

impl AttributeKeyStr {
    /// Converts a `&str` into a `&AttributeKeyStr`.
    /// 
    /// # Safety
    /// `AttributeKeyStr` is a transparent wrapper over `str`
    pub fn from_str(s: &str) -> &Self {
        unsafe { &*(s as *const str as *const Self) }
    }

    /// Returns the underlying string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns an owned version of this key.
    pub fn to_attribute_key_buf(&self) -> AttributeKeyBuf {
        AttributeKeyBuf::new(self.as_str())
    }

    /// Returns the key as an owned `String`.
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToOwned for AttributeKeyStr {
    type Owned = AttributeKeyBuf;
    fn to_owned(&self) -> Self::Owned {
        self.to_attribute_key_buf()
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE KEYS — REFERENCE WRAPPER
// ————————————————————————————————————————————————————————————————————————————

/// A flexible reference to an attribute key (`borrowed or owned`).
pub struct AttributeKeyRef<'a>(Cow<'a, AttributeKeyStr>);

impl<'a> AttributeKeyRef<'a> {
    /// Creates a borrowed reference to an attribute key.
    pub fn borrowed(s: &'a AttributeKeyStr) -> Self {
        Self(Cow::Borrowed(s))
    }

    /// Creates an owned reference from any string-like value.
    pub fn owned(value: impl Into<String>) -> Self {
        let buf = AttributeKeyBuf::new(value);
        Self(Cow::Owned(buf.into()))
    }

    /// Returns the `AttributeKeyStr` representation.
    pub fn as_key_str(&self) -> &AttributeKeyStr {
        self.0.as_ref()
    }

    /// Returns the raw `&str` slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> From<&'a AttributeKeyStr> for AttributeKeyRef<'a> {
    fn from(value: &'a AttributeKeyStr) -> Self {
        Self::borrowed(value)
    }
}

impl From<AttributeKeyBuf> for AttributeKeyRef<'_> {
    fn from(value: AttributeKeyBuf) -> Self {
        Self(Cow::Owned(value.into()))
    }
}

impl<'a> Debug for AttributeKeyRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<'a> Display for AttributeKeyRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE KEYS — OWNED API
// ————————————————————————————————————————————————————————————————————————————

/// An owned attribute key.
#[derive(Clone)]
pub struct AttributeKeyBuf(String);

impl AttributeKeyBuf {
    /// Creates a new owned attribute key.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns a borrowed string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a borrowed `AttributeKeyStr`.
    pub fn as_attribute_key_str(&self) -> &AttributeKeyStr {
        AttributeKeyStr::from_str(self.as_str())
    }
}

impl Debug for AttributeKeyBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for AttributeKeyBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// COMMON TRAIT IMPLEMENTATIONS
// ————————————————————————————————————————————————————————————————————————————

impl From<String> for AttributeKeyBuf {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for AttributeKeyBuf {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<&AttributeKeyStr> for AttributeKeyBuf {
    fn from(s: &AttributeKeyStr) -> Self {
        Self::new(s.as_str())
    }
}

impl From<AttributeKeyBuf> for String {
    fn from(buf: AttributeKeyBuf) -> Self {
        buf.0
    }
}

impl Deref for AttributeKeyBuf {
    type Target = AttributeKeyStr;
    fn deref(&self) -> &Self::Target {
        self.as_attribute_key_str()
    }
}

impl AsRef<str> for AttributeKeyBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<AttributeKeyStr> for AttributeKeyBuf {
    fn as_ref(&self) -> &AttributeKeyStr {
        self.as_attribute_key_str()
    }
}

impl Borrow<AttributeKeyStr> for AttributeKeyBuf {
    fn borrow(&self) -> &AttributeKeyStr {
        self.as_attribute_key_str()
    }
}

impl AsRef<AttributeKeyStr> for str {
    fn as_ref(&self) -> &AttributeKeyStr {
        AttributeKeyStr::from_str(self)
    }
}

impl PartialEq for AttributeKeyBuf {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for AttributeKeyBuf {}

impl PartialOrd for AttributeKeyBuf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for AttributeKeyBuf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for AttributeKeyBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

// `AttributeKeyStr` trait impls
impl PartialEq for AttributeKeyStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for AttributeKeyStr {}

impl PartialOrd for AttributeKeyStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for AttributeKeyStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for AttributeKeyStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

// Flexible equality for comparisons
impl PartialEq<AttributeKeyStr> for str {
    fn eq(&self, other: &AttributeKeyStr) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for AttributeKeyStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for AttributeKeyStr {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<AttributeKeyBuf> for AttributeKeyStr {
    fn eq(&self, other: &AttributeKeyBuf) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<AttributeKeyStr> for AttributeKeyBuf {
    fn eq(&self, other: &AttributeKeyStr) -> bool {
        self.as_str() == other.as_str()
    }
}


// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE VALUES
// ————————————————————————————————————————————————————————————————————————————

#[derive(Clone)]
pub enum AttributeValueBuf {
    Literal(String),
}

impl AttributeValueBuf {
    pub fn literal(value: impl Into<String>) -> Self {
        AttributeValueBuf::Literal(value.into())
    }
    pub fn as_str(&self) -> &str {
        match self {
            AttributeValueBuf::Literal(x) => x,
        }
    }
    pub fn as_mut_string(&mut self) -> &mut String {
        match self {
            AttributeValueBuf::Literal(x) => x,
        }
    }
}

impl Debug for AttributeValueBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.as_str(), f)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP
// ————————————————————————————————————————————————————————————————————————————

#[derive(Clone, Default)]
pub struct AttributeMap(IndexMap<AttributeKeyBuf, AttributeValueBuf>);

impl Debug for AttributeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP — CONSTRUCTION
// ————————————————————————————————————————————————————————————————————————————

impl<K: Into<AttributeKeyBuf>, V: Into<AttributeValueBuf>> FromIterator<(K, V)> for AttributeMap {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let map = iter
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self(map)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP — INDEXING
// ————————————————————————————————————————————————————————————————————————————

impl Index<&str> for AttributeMap {
    type Output = AttributeValueBuf;

    fn index(&self, key: &str) -> &Self::Output {
        &self.0[AttributeKeyStr::from_str(key)]
    }
}

impl IndexMut<&str> for AttributeMap {
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        self.0
            .get_mut(AttributeKeyStr::from_str(key))
            .expect("key not found")
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP — MISCELLANEOUS
// ————————————————————————————————————————————————————————————————————————————

impl AttributeMap {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<Q: AsRef<str>>(&self, key: Q) -> Option<&AttributeValueBuf> {
        self.0.get(AttributeKeyStr::from_str(key.as_ref()))
    }

    pub fn get_mut<Q: AsRef<str>>(&mut self, key: Q) -> Option<&mut AttributeValueBuf> {
        self.0.get_mut(AttributeKeyStr::from_str(key.as_ref()))
    }

    pub fn insert<K: Into<AttributeKeyBuf>, V: Into<AttributeValueBuf>>(
        &mut self,
        key: K,
        value: V,
    ) -> Option<AttributeValueBuf> {
        self.0.insert(key.into(), value.into())
    }

    pub fn remove<Q: AsRef<str>>(&mut self, key: Q) -> Option<AttributeValueBuf> {
        self.0
            .swap_remove(AttributeKeyStr::from_str(key.as_ref()))
    }

    pub fn contains_key<Q: AsRef<str>>(&self, key: Q) -> bool {
        self.0.contains_key(AttributeKeyStr::from_str(key.as_ref()))
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn keys(&self) -> impl Iterator<Item = &AttributeKeyBuf> {
        self.0.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &AttributeValueBuf> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut AttributeValueBuf> {
        self.0.values_mut()
    }

    /// Inserts the given string literal if the key does not exist,
    /// and returns a mutable reference to the value.
    pub fn get_or_insert_literal<Q: AsRef<str>>(
        &mut self,
        key: Q,
        default: impl Into<String>,
    ) -> &mut AttributeValueBuf {
        let key_ref = AttributeKeyStr::from_str(key.as_ref());

        if self.0.contains_key(key_ref) {
            // Lookup separately without early return to appease borrow checker
            return self.0.get_mut(key_ref).unwrap();
        }

        // Safe to insert now
        let key_buf = AttributeKeyBuf::from(key_ref);
        self.0.entry(key_buf).or_insert_with(|| AttributeValueBuf::literal(default))
    }

    /// Merges another `AttributeMap` into this one, overwriting existing keys.
    pub fn merge(&mut self, other: &AttributeMap) {
        for (key, value) in other.iter() {
            self.0.insert(key.clone(), value.clone());
        }
    }

    pub fn extend(&mut self, other: AttributeMap) {
        self.0.extend(other);
    }

    /// Merges another `AttributeMap` into this one, but does NOT overwrite existing keys.
    pub fn merge_if_absent(&mut self, other: &AttributeMap) {
        for (key, value) in other.iter() {
            self.0.entry(key.clone()).or_insert_with(|| value.clone());
        }
    }
}



// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP — ENTRY
// ————————————————————————————————————————————————————————————————————————————

/// A map entry for mutation via `entry()` API.
pub enum AttributeMapEntry<'a> {
    Occupied(OccupiedAttributeEntry<'a>),
    Vacant(VacantAttributeEntry<'a>),
}

pub struct OccupiedAttributeEntry<'a> {
    inner: indexmap::map::OccupiedEntry<'a, AttributeKeyBuf, AttributeValueBuf>,
}

pub struct VacantAttributeEntry<'a> {
    inner: indexmap::map::VacantEntry<'a, AttributeKeyBuf, AttributeValueBuf>,
}

impl<'a> AttributeMapEntry<'a> {
    /// Returns `true` if the entry is occupied.
    pub fn is_occupied(&self) -> bool {
        matches!(self, AttributeMapEntry::Occupied(_))
    }

    /// Returns `true` if the entry is vacant.
    pub fn is_vacant(&self) -> bool {
        matches!(self, AttributeMapEntry::Vacant(_))
    }

    /// Returns the value if the entry is occupied.
    pub fn get(&self) -> Option<&AttributeValueBuf> {
        match self {
            AttributeMapEntry::Occupied(e) => Some(e.get()),
            AttributeMapEntry::Vacant(_) => None,
        }
    }

    /// Inserts a value into the entry and returns a mutable reference.
    pub fn or_insert(self, default: AttributeValueBuf) -> &'a mut AttributeValueBuf {
        match self {
            AttributeMapEntry::Occupied(e) => e.into_mut(),
            AttributeMapEntry::Vacant(e) => e.insert(default),
        }
    }

    /// Inserts a string literal if vacant and returns a mutable reference.
    pub fn or_insert_literal(self, default: impl Into<String>) -> &'a mut AttributeValueBuf {
        self.or_insert(AttributeValueBuf::literal(default))
    }
}

impl<'a> OccupiedAttributeEntry<'a> {
    pub fn get(&self) -> &AttributeValueBuf {
        self.inner.get()
    }

    pub fn get_mut(&mut self) -> &mut AttributeValueBuf {
        self.inner.get_mut()
    }

    pub fn into_mut(self) -> &'a mut AttributeValueBuf {
        self.inner.into_mut()
    }

    pub fn remove(self) -> AttributeValueBuf {
        self.inner.swap_remove()
    }

    pub fn key(&self) -> &AttributeKeyBuf {
        self.inner.key()
    }
}

impl<'a> VacantAttributeEntry<'a> {
    pub fn insert(self, value: AttributeValueBuf) -> &'a mut AttributeValueBuf {
        self.inner.insert(value)
    }

    pub fn key(&self) -> &AttributeKeyBuf {
        self.inner.key()
    }
}

impl AttributeMap {
    /// Gets the entry for the given key, allowing efficient mutation or insertion.
    pub fn entry<Q: AsRef<str>>(&mut self, key: Q) -> AttributeMapEntry<'_> {
        let key_ref = AttributeKeyStr::from_str(key.as_ref());
        match self.0.entry(AttributeKeyBuf::from(key_ref)) {
            indexmap::map::Entry::Occupied(e) => {
                AttributeMapEntry::Occupied(OccupiedAttributeEntry { inner: e })
            }
            indexmap::map::Entry::Vacant(e) => {
                AttributeMapEntry::Vacant(VacantAttributeEntry { inner: e })
            }
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ATTRIBUTE MAP — ITERATORS
// ————————————————————————————————————————————————————————————————————————————

// These are public so users can name them, but they don’t expose IndexMap

pub struct AttributeMapIntoIter(indexmap::map::IntoIter<AttributeKeyBuf, AttributeValueBuf>);
pub struct AttributeMapIter<'a>(indexmap::map::Iter<'a, AttributeKeyBuf, AttributeValueBuf>);
pub struct AttributeMapIterMut<'a>(indexmap::map::IterMut<'a, AttributeKeyBuf, AttributeValueBuf>);

impl Iterator for AttributeMapIntoIter {
    type Item = (AttributeKeyBuf, AttributeValueBuf);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for AttributeMapIter<'a> {
    type Item = (&'a AttributeKeyBuf, &'a AttributeValueBuf);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for AttributeMapIterMut<'a> {
    type Item = (&'a AttributeKeyBuf, &'a mut AttributeValueBuf);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl IntoIterator for AttributeMap {
    type Item = (AttributeKeyBuf, AttributeValueBuf);
    type IntoIter = AttributeMapIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        AttributeMapIntoIter(self.0.into_iter())
    }
}

impl<'a> IntoIterator for &'a AttributeMap {
    type Item = (&'a AttributeKeyBuf, &'a AttributeValueBuf);
    type IntoIter = AttributeMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AttributeMapIter(self.0.iter())
    }
}

impl<'a> IntoIterator for &'a mut AttributeMap {
    type Item = (&'a AttributeKeyBuf, &'a mut AttributeValueBuf);
    type IntoIter = AttributeMapIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AttributeMapIterMut(self.0.iter_mut())
    }
}

impl AttributeMap {
    pub fn iter(&self) -> impl Iterator<Item = (&AttributeKeyBuf, &AttributeValueBuf)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&AttributeKeyBuf, &mut AttributeValueBuf)> {
        self.0.iter_mut()
    }

    pub fn into_iter_erased(self) -> impl Iterator<Item = (AttributeKeyBuf, AttributeValueBuf)> {
        self.0.into_iter()
    }
}
