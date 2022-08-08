use std::{marker::PhantomData, ops::Index};

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

impl<K: Cardinality, V> EnumMap<K, V> {
    const USED_BITS: StorageSize = (1 << K::CARDINALITY) - 1;
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

    /*
    fn insert(&mut self, key: K, value: V) -> bool
    where
        V: PartialEq
    {
        let pointer = &mut self.0[key.enum_to_index() as usize];
        let old = pointer;
        let new = Some(value);
        pointer = new;
        old != new
    }


    fn remove(&mut self, key: K) -> bool
    where
        V: PartialEq
    {
        // self.0[key.enum_to_index() as usize] = None;
        let pointer = &self.0[key.enum_to_index() as usize];
        let old = *pointer;
        *pointer = None;
        old != None
    }
    */

    fn clear(&mut self) {
        self.0.clear()
    }

    fn get(&self, key: K) -> Option<&V> {
        self.0[key.enum_to_index() as usize].as_ref()
    }

    // fn union(self, other: &Self) -> Self {
    //     self.into_iter().zip(other.iter()).map(|(x, y)| x.and(y)).collect()
    // }
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
