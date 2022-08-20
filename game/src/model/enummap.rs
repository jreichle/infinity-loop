use std::{marker::PhantomData, ops::Index};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite};

///!-------------------
///! UNDER CONSTRUCTION
///!-------------------

// alternative representation
// pub struct EnumMap<K, V>([Option<V>; StorageSize::BITS as usize], PhantomData<K>);

/// underlying integer type
type StorageSize = u64;

/// dense structure, preallocates all memory instead of growing dynamically
///
/// for use case in enums and assuming even distribution,
/// 87.5% of inhabitants require vec of size of half or more of total capacity
///
/// one notable exception is single element sets
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct EnumMap<K, V>(Vec<Option<V>>, PhantomData<K>);

impl<K, V: Clone> Clone for EnumMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<K: Cardinality, V: Clone> Default for EnumMap<K, V> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<K: Cardinality, V: Clone> EnumMap<K, V> {
    fn empty() -> Self {
        Self(vec![None; K::CARDINALITY as usize], PhantomData)
    }
}

impl<K: Finite, V: Clone> EnumMap<K, V> {
    fn inserted(self, key: K, value: V) -> Self {
        let mut set = self.clone();
        set.0[key.enum_to_index() as usize] = Some(value);
        set
    }

    fn removed(self, key: K) -> Self {
        let mut set = self.clone();
        set.0[key.enum_to_index() as usize] = None;
        set
    }
}

impl<K: Finite, V> EnumMap<K, V> {
    fn contains(self, key: K) -> bool {
        self.get(key).is_some()
    }

    fn insert(&mut self, key: K, value: V) {
        self.0[key.enum_to_index() as usize] = Some(value);
    }

    fn remove(&mut self, key: K) {
        self.0[key.enum_to_index() as usize] = None;
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn get(&self, key: K) -> Option<&V> {
        self.0[key.enum_to_index() as usize].as_ref()
    }

    fn intersection(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter()
                .zip(other.0.into_iter())
                .map(|(x, y)| x.or(y))
                .collect(),
            PhantomData,
        )
    }

    fn union(self, other: Self) -> Self {
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

impl<K: Finite, V> Index<K> for EnumMap<K, V> {
    type Output = Option<V>;

    fn index(&self, index: K) -> &Self::Output {
        &self.0[index.enum_to_index() as usize]
    }
}

impl<K: Finite, V: Clone> FromIterator<(K, V)> for EnumMap<K, V> {
    // TODO: use faster insert
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        iter.into_iter()
            .fold(Self::empty(), |acc, (k, v)| acc.inserted(k, v))
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
    fn insert_then_contains(mut map: EnumMap<Max<20>, u32>, key: Max<20>, value: u32) -> bool {
        map.insert(key, value);
        map.contains(key)
    }

    #[quickcheck]
    fn remove_then_does_not_contains(mut map: EnumMap<Max<20>, u32>, key: Max<20>) -> bool {
        map.remove(key);
        !map.contains(key)
    }
}
