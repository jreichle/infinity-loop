use std::{
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite};

/// dense structure, preallocates all memory instead of growing dynamically
///
/// for use case in enums and assuming even distribution,
/// 87.5% of inhabitants require a vector of size of half or more of total capacity
///
/// one notable exception are maps with a single key
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
    pub fn empty() -> Self {
        Self(vec![None; K::CARDINALITY as usize], PhantomData)
    }
}

impl<K, V> EnumMap<K, V> {
    pub fn clear(&mut self) {
        self.0.fill_with(|| None)
    }
}

impl<K: Finite, V> EnumMap<K, V> {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(Option::is_none)
    }

    pub fn len(&self) -> u32 {
        self.0.iter().filter(|x| x.is_some()).count() as u32
    }

    pub fn contains(self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0[key.enum_to_index() as usize] = Some(value);
    }

    pub fn remove(&mut self, key: K) {
        self.0[key.enum_to_index() as usize] = None;
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.0[key.enum_to_index() as usize].as_ref()
    }

    pub fn intersection(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(other.0.into_iter())
                .map(|(x, y)| x.or(y))
                .collect(),
            PhantomData,
        )
    }

    pub fn union(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(other.0.into_iter())
                .map(|(x, y)| x.and(y))
                .collect(),
            PhantomData,
        )
    }
}

impl<K: Finite, V: Clone> EnumMap<K, V> {
    pub fn inserted(self, key: K, value: V) -> Self {
        let mut set = self;
        set.0[key.enum_to_index() as usize] = Some(value);
        set
    }

    pub fn removed(self, key: K) -> Self {
        let mut set = self;
        set.0[key.enum_to_index() as usize] = None;
        set
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            elements: self.0.clone(),
            index: 0,
            phantom: PhantomData,
        }
    }

    pub fn into_keys(&self) -> impl Iterator<Item = K> {
        self.iter().map(|t| t.0)
    }

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
    // TODO: use faster insert
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = Self::empty();
        iter.into_iter().for_each(|(k, v)| map.insert(k, v));
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
                .map(|_| Option::<V>::arbitrary(g))
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
