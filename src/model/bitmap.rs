use std::{borrow::Borrow, iter::repeat, marker::PhantomData, ops::Index};

use super::{cardinality::Cardinality, finite::Finite};

///!-------------------
///! UNDER CONSTRUCTION
///!-------------------

// alternative representation
// pub struct BitMap<K, V>([Option<V>; BitMapSize::BITS as usize], PhantomData<K>);

/// underlying integer type
type BitMapSize = u64;

/// dense structure, preallocates all memory instead of growing dynamically
///
/// for use case in enums and assuming even distribution,
/// 87.5% of inhabitants require vec with capacity of half or more of total capacity
///
/// one notable exception is single element sets
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct BitMap<K, V>(Vec<Option<V>>, PhantomData<K>);

impl<K, V: Clone> Clone for BitMap<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<K: Cardinality, V: Clone> Default for BitMap<K, V> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<K: Cardinality, V> BitMap<K, V> {
    const USED_BITS: BitMapSize = (1 << K::CARDINALITY) - 1;
}

impl<K: Cardinality, V: Clone> BitMap<K, V> {
    fn empty() -> Self {
        Self(
            repeat(None).take(K::CARDINALITY as usize).collect(),
            PhantomData,
        )
    }
}

impl<K: Finite, V: Clone> BitMap<K, V> {
    fn inserted(&self, key: K, value: V) -> Self {
        let mut set = Self::empty();
        set.insert(key, value);
        set
    }
}

impl<K: Finite, V> BitMap<K, V> {
    fn contains(self, key: K) -> bool {
        self.0[key.enum_to_index() as usize].is_some()
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

    fn get<B: Borrow<K> + Finite>(&self, _key: &B) -> Option<&V> {
        // self.0[key.enum_to_index() as usize];
        todo!()
    }
}

impl<K: Finite, V> Index<K> for BitMap<K, V> {
    type Output = Option<V>;

    fn index(&self, index: K) -> &Self::Output {
        &self.0[index.enum_to_index() as usize]
    }
}
