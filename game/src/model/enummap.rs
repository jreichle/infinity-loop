use std::{
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite};

/// Dense Map data structure with keys of a statically enumerable types with known cardinality, as witnessed by the traits [Cardinality] and [Finite]
///
/// The capacity is determined at compile-time by the type of the stored elements and precludes the need for user management
///
/// This implementation preallocates a fixed amount of memory and does not grow dynamically
///
/// # Memory Efficiency
/// * assuming even distribution 87.5% of inhabitants require a vector of size of half or more of total capacity
/// * inefficient representation for singleton maps
///
/// # Invariant
///
/// `∀m : EnumMap<K, _>. m.0.len() ≡ K::CARDINALITY()`
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EnumMap<K, V>(Vec<Option<V>>, PhantomData<K>);

impl<K, V: Clone> Clone for EnumMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<K: Cardinality, V: Clone> Default for EnumMap<K, V> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<K: Cardinality, V: Clone> EnumMap<K, V> {
    /// Create an empty map
    pub fn empty() -> Self {
        Self(vec![None; K::CARDINALITY as usize], PhantomData)
    }
}

impl<K, V> EnumMap<K, V> {
    /// Wipes all stored associations
    pub fn clear(&mut self) {
        self.0.fill_with(|| None)
    }
}

impl<K: Finite, V> EnumMap<K, V> {
    /// Indicates if the map contains 0 associations
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(Option::is_none)
    }

    /// Returns number of associations in the map
    pub fn len(&self) -> u32 {
        self.0.iter().filter(|x| x.is_some()).count() as u32
    }

    /// Checks if the map contains a given key
    pub fn contains(self, key: K) -> bool {
        self.get(key).is_some()
    }

    /// Inserts given association into the map
    ///
    /// Mutable variant of [`EnumMap::inserted`]
    pub fn insert(&mut self, key: K, value: V) {
        self.0[key.enum_to_index() as usize] = Some(value);
    }

    /// Removes given key from the map
    ///
    /// Mutable variant of [`EnumMap::removed`]
    pub fn remove(&mut self, key: K) {
        self.0[key.enum_to_index() as usize] = None;
    }

    /// Queries the associated value of the given key
    pub fn get(&self, key: K) -> Option<&V> {
        self.0[key.enum_to_index() as usize].as_ref()
    }

    /// Returns a map containing every key present in both maps, favoring associated values of this map
    pub fn intersection(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(other.0.into_iter())
                .map(|(x, y)| y.and(x))
                .collect(),
            PhantomData,
        )
    }

    /// Returns a map containing every key present in either map, favoring associated values of this map
    pub fn union(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(other.0.into_iter())
                .map(|(x, y)| x.or(y))
                .collect(),
            PhantomData,
        )
    }
}

impl<K: Finite, V: Clone> EnumMap<K, V> {
    /// Inserts given associations into the map
    ///
    /// Immutable variant of [`EnumMap::insert`]
    pub fn inserted(self, key: K, value: V) -> Self {
        let mut set = self;
        set.0[key.enum_to_index() as usize] = Some(value);
        set
    }

    /// Removes given kay from the map
    ///
    /// Immutable variant of [`EnumMap::remove`]
    pub fn removed(self, key: K) -> Self {
        let mut set = self;
        set.0[key.enum_to_index() as usize] = None;
        set
    }

    /// Returns an iterator over the associations in the map
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            elements: self.0.clone(),
            index: 0,
            phantom: PhantomData,
        }
    }

    /// Returns an iterator over all keys in the map
    pub fn into_keys(&self) -> impl Iterator<Item = K> {
        self.iter().map(|t| t.0)
    }

    /// Returns an iterator over all associated values in the map
    pub fn into_values(&self) -> impl Iterator<Item = V> {
        self.iter().map(|t| t.1)
    }
}

impl<K: Finite, V> Index<K> for EnumMap<K, V> {
    type Output = Option<V>;

    fn index(&self, index: K) -> &Self::Output {
        &self.0[index.enum_to_index() as usize]
    }
}

impl<K: Finite, V> IndexMut<K> for EnumMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.0[index.enum_to_index() as usize]
    }
}

/// Iterator for [`EnumMap`]
///
/// the successive keys in the iterator are in strictly ascending order iff the EnumSet is order isomorphic
pub struct Iter<K, V> {
    elements: Vec<Option<V>>,
    index: usize,
    phantom: PhantomData<K>,
}

impl<K: Finite, V: Clone> IntoIterator for EnumMap<K, V> {
    type Item = (K, V);

    type IntoIter = Iter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            elements: self.0,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<K: Finite, V: Clone> Iterator for Iter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while (self.index as u64) < K::CARDINALITY && self.elements[self.index].is_none() {
            self.index += 1
        }
        if self.index as u64 == K::CARDINALITY {
            None
        } else {
            let old_index = self.index;
            self.index += 1;
            Some((
                K::unchecked_index_to_enum(old_index as u64),
                self.elements[old_index].clone().unwrap(),
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.elements[self.index..]
            .iter()
            .filter(|x| x.is_some())
            .count() as usize;
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.elements[self.index..]
            .iter()
            .filter(|x| x.is_some())
            .count() as usize
    }
}

impl<K: Finite, V: Clone> FusedIterator for Iter<K, V> {}

impl<K: Finite, V: Clone> FromIterator<(K, V)> for EnumMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::empty();
        map.extend(iter);
        map
    }
}

impl<K: Finite, V> Extend<(K, V)> for EnumMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(k, v)| self.insert(k, v));
    }
}

impl<K: Cardinality + 'static, V: Arbitrary> Arbitrary for EnumMap<K, V> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self(
            (0..K::CARDINALITY)
                .map(|_| Arbitrary::arbitrary(g))
                .collect(),
            PhantomData,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::model::{enummap::EnumMap, interval::Max};

    #[quickcheck]
    fn empty_is_all_none() -> bool {
        EnumMap::<u8, bool>::empty().0.iter().all(Option::is_none)
    }

    #[quickcheck]
    fn empty_then_is_empty() -> bool {
        EnumMap::<Max<20>, u32>::empty().is_empty()
    }

    #[quickcheck]
    fn is_empty_iff_len_is_zero(map: EnumMap<Max<20>, u32>) -> bool {
        map.is_empty() == (map.len() == 0)
    }

    #[quickcheck]
    fn clear_then_is_empty(mut map: EnumMap<Max<20>, u32>) -> bool {
        map.clear();
        map.is_empty()
    }

    #[quickcheck]
    fn insert_then_contains(mut map: EnumMap<Max<20>, u32>, key: Max<20>, value: u32) -> bool {
        map.insert(key, value);
        map.contains(key)
    }

    #[quickcheck]
    fn remove_then_does_not_contains(mut map: EnumMap<Max<20>, u32>, key: Max<20>) -> bool {
        map.remove(key);
        !map.contains(key)
    }

    #[quickcheck]
    fn into_iter_then_collect_is_id(map: EnumMap<Max<20>, u32>) -> bool {
        map.clone().into_iter().collect::<EnumMap<_, _>>() == map
    }

    #[quickcheck]
    fn iter_size_hint_is_exact(map: EnumMap<Max<20>, u32>, random: usize) -> bool {
        let skip_distance = random.checked_rem(map.len() as usize).unwrap_or_default();
        let len = map.iter().skip(skip_distance).collect::<Vec<_>>().len();
        map.iter().skip(skip_distance).size_hint() == (len, Some(len))
    }

    #[quickcheck]
    fn iter_count_is_correct(map: EnumMap<Max<20>, u32>, random: usize) -> bool {
        let skip_distance = random.checked_rem(map.len() as usize).unwrap_or_default();
        map.iter().skip(skip_distance).count()
            == map.iter().skip(skip_distance).collect::<Vec<_>>().len()
    }
}
