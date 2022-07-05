use std::{fmt::Display, hash::Hash, marker::PhantomData, ops::Not};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite, lattice::BoundedLattice};

// prefered representation
// type Width = u64;
// struct BitSet<A: Cardinality>([Width; (A::CARDINALITY + Width::BITS - 1) / Width::BITS], PhantomData<A>);

/// number of bits equals number of storable elements
type BitSetSize = u64;

// compile-time proof that BitSet can store 64 elements
const _: () = {
    type Has64Inhabitants = BitSet<(bool, Option<bool>)>;
    assert!(Has64Inhabitants::CARDINALITY == 64);
    assert!(BitSet::<Has64Inhabitants>::USED_BITS == u64::MAX)
};

/// Set for storing elements of statically enumerable types with known cardinality, as witnessed by the traits [Cardinality] and [Finite]
///
/// the implementation uses a fixed amount of memory and does not grow dynamically
///
/// trying to use types that exceed the storing capacity leads to a compile-time error
///
/// invariant ensuring canonical representation: bits exceeding A::Cardinality are always 0
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BitSet<A>(BitSetSize, PhantomData<A>);

// ability to [Clone] independent of generic argument
impl<A> Clone for BitSet<A> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

// ability to [Copy] independent of generic argument
impl<A> Copy for BitSet<A> {}

impl<A: Display + Finite> Display for BitSet<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = self
            .into_iter()
            .fold(String::new(), |acc, x| acc + x.to_string().as_str() + ", ");
        string.pop();
        string.pop();
        write!(f, "{{{}}}", string)
    }
}

impl<A: Cardinality> Cardinality for BitSet<A> {
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}

impl<A: Finite> Finite for BitSet<A> {
    fn index_to_enum(value: u64) -> Self {
        Self(value & Self::USED_BITS, PhantomData) // truncating
    }

    fn enum_to_index(&self) -> u64 {
        self.0
    }
}

impl<A> BitSet<A> {
    /// set containing 0 elements
    pub const EMPTY: Self = Self(BitSetSize::MIN, PhantomData);

    /// indicates if the set contains 0 elements
    pub const fn is_empty(self) -> bool {
        self.0 == BitSetSize::MIN
    }

    /// returns number of elements in the set
    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// returns a set containing every element present in both sets
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0, PhantomData)
    }

    /// returns a set containing any elements present in either set
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0, PhantomData)
    }

    /// returns a set containing all elements in the first set without the elements in the second set
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0, PhantomData)
    }

    /// returns a set containing all the elements that are contained in exactly one set
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0, PhantomData)
    }

    /// indicates if the other set contains at least all elements of this one
    pub const fn is_subset(self, other: Self) -> bool {
        self.0 | other.0 == self.0
    }

    /// indicates if this set contains at least all elements of the other one
    pub const fn is_superset(self, other: Self) -> bool {
        other.is_subset(self)
    }

    /// indicates if both sets share no common elements
    pub const fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == BitSetSize::MIN
    }
}

impl<A: Cardinality> BitSet<A> {
    /// mask of all used bits
    ///
    /// laws:
    /// * `∀s: BitSet. s.0 & USED_BITS == s.0`
    /// * equivalent: `∀s: BitSet. s.intersection(BitSet::FULL) == s`
    const USED_BITS: BitSetSize = if A::CARDINALITY <= BitSetSize::BITS as BitSetSize {
        // == `1 << A:CARDINALITY - 1` without overflow
        BitSetSize::MAX >> (BitSetSize::BITS as BitSetSize - A::CARDINALITY)
    } else {
        panic!("BitSet only supports up to 64 elements")
    };

    /// set containing all possible elements
    pub const FULL: Self = Self(Self::USED_BITS, PhantomData);

    /// returns a set containing all elements not in this set
    ///
    /// mutable variant of [not]
    pub fn complement(mut self) {
        self.0 = !self.0 & Self::USED_BITS
    }
}

impl<A: Finite> BitSet<A> {
    /// set only containing the given element
    pub fn singleton(element: A) -> Self {
        Self::EMPTY.inserted(element)
    }

    /// checks if set contains a given element
    pub fn contains(self, element: A) -> bool {
        test_bit(self.0, element.enum_to_index())
    }

    /// inserts given element into the set
    ///
    /// immutable variant of [insert]
    pub fn inserted(self, element: A) -> Self {
        Self(set_bit(self.0, element.enum_to_index()), PhantomData)
    }

    /// removes given element from the set
    ///
    /// immutable variant of [remove]
    pub fn removed(self, element: A) -> Self {
        Self(clear_bit(self.0, element.enum_to_index()), PhantomData)
    }

    /// toggles given element in the set
    ///
    /// immutable variant of [toggle]
    pub fn toggled(self, element: A) -> Self {
        Self(toggle_bit(self.0, element.enum_to_index()), PhantomData)
    }

    /// inserts given element into the set and indicates if the set has changed
    ///
    /// mutable variant of [inserted]
    pub fn insert(mut self, element: A) -> bool {
        let old = self.0;
        self.0 = set_bit(self.0, element.enum_to_index());
        self.0 != old
    }

    /// removes given element from the set and indicates if the set has changed
    ///
    /// mutable variant of [removed]
    pub fn remove(mut self, element: A) -> bool {
        let old = self.0;
        self.0 = clear_bit(self.0, element.enum_to_index());
        self.0 != old
    }

    /// toggles given element in the set
    ///
    /// mutable variant of [toggled]
    pub fn toggle(mut self, element: A) {
        self.0 = toggle_bit(self.0, element.enum_to_index());
    }

    /// unwraps the only element of the set
    ///
    /// * return `Some(e)` if set contains `e` as only element
    /// * returns `None` if set contains several elements or is empty
    pub fn unwrap_if_singleton(self) -> Option<A> {
        if self.len() == 1 {
            Some(A::index_to_enum(self.0.trailing_zeros() as BitSetSize))
        } else {
            None
        }
    }

    /// reference is sufficient to construct iterator
    pub fn iter(&self) -> Iter<A> {
        Iter {
            bits: self.0,
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl<A: Finite> IntoIterator for BitSet<A> {
    type Item = A;

    type IntoIter = Iter<A>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            bits: self.0,
            index: 0,
            phantom: PhantomData,
        }
    }
}

pub struct Iter<A> {
    bits: BitSetSize,
    index: BitSetSize,
    phantom: PhantomData<A>,
}

impl<A: Finite> Iterator for Iter<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }

        let trailing = self.bits.trailing_zeros() as BitSetSize + 1;
        self.bits >>= trailing;
        self.index += trailing;

        Some(A::index_to_enum(self.index - 1))
    }
}

impl<A: Finite> FromIterator<A> for BitSet<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter().fold(Self::EMPTY, Self::inserted)
    }
}

impl<A: Finite> Extend<A> for BitSet<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter().for_each(|e| {
            self.insert(e);
        })
    }
}

impl<A: Cardinality> Not for BitSet<A> {
    type Output = Self;

    /// returns a set containing all elements not in this set
    ///
    /// immutable variant of [complement]
    fn not(self) -> Self::Output {
        Self(!self.0 & Self::USED_BITS, PhantomData)
    }
}

impl<A: Arbitrary + Finite> Arbitrary for BitSet<A> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Vec::arbitrary(g).into_iter().collect()
    }
}

impl<A: Cardinality> BoundedLattice for BitSet<A> {
    fn bottom() -> Self {
        Self::EMPTY
    }

    fn top() -> Self {
        Self::FULL
    }

    fn meet(self, other: Self) -> Self {
        self.intersection(other)
    }

    fn join(self, other: Self) -> Self {
        self.union(other)
    }
}

#[inline(always)]
fn bit_at(index: u64) -> u64 {
    1 << index
}

#[inline(always)]
fn test_bit(value: u64, index: u64) -> bool {
    value & bit_at(index) != 0
}

#[inline(always)]
fn set_bit(value: u64, index: u64) -> u64 {
    value | bit_at(index)
}

#[inline(always)]
fn clear_bit(value: u64, index: u64) -> u64 {
    value & !bit_at(index)
}

#[inline(always)]
fn toggle_bit(value: u64, index: u64) -> u64 {
    value ^ bit_at(index)
}

#[cfg(test)]
mod test {

    use enumset::EnumSet;

    use crate::model::{bitset::*, tile::Square};

    #[quickcheck]
    fn id(set: BitSet<BitSet<Option<bool>>>) -> bool {
        set == BitSet::index_to_enum(set.enum_to_index())
    }

    #[quickcheck]
    fn id2(value: u8) -> bool {
        type Set = BitSet<EnumSet<Square>>;
        let value = value as u64 % Set::CARDINALITY;
        value == Set::index_to_enum(value).enum_to_index()
    }

    #[quickcheck]
    fn bitshift_and_trailing_zeroes_is_id(index: u8) -> bool {
        let index = (index % u64::BITS as u8) as u64;
        (1u64 << index).trailing_zeros() as u64 == index
    }

    #[quickcheck]
    fn insert_then_contains(set: BitSet<BitSet<bool>>, element: BitSet<bool>) -> bool {
        set.inserted(element).contains(element)
    }

    #[quickcheck]
    fn remove_then_does_not_contain(set: BitSet<BitSet<bool>>, element: BitSet<bool>) -> bool {
        !set.removed(element).contains(element)
    }

    #[quickcheck]
    fn singleton_then_unwrap_if_singleton_always_succeeds(element: BitSet<bool>) -> bool {
        BitSet::singleton(element).unwrap_if_singleton() == Some(element)
    }

    #[quickcheck]
    fn unwrap_if_singleton_check(set: BitSet<BitSet<bool>>) -> bool {
        match set.unwrap_if_singleton() {
            None => set.is_empty() || set.len() > 1,
            Some(_) => set.len() == 1,
        }
    }

    #[quickcheck]
    fn bitset_invariant(set: BitSet<BitSet<bool>>) -> bool {
        set.0 & BitSet::<BitSet<bool>>::USED_BITS == set.0
            && set.intersection(BitSet::FULL) == set
            && set.union(BitSet::EMPTY) == set
    }

    #[quickcheck]
    fn set_and_not_set_is_disjoint(set: BitSet<BitSet<bool>>) -> bool {
        set.is_disjoint(!set)
    }

    #[quickcheck]
    fn iterator_of_singleton_set_contains_single_element(element: BitSet<bool>) -> bool {
        let mut iter = BitSet::singleton(element).iter();
        iter.next() == Some(element) && iter.next() == None
    }

    #[quickcheck]
    fn iter_then_collect_is_id(set: BitSet<BitSet<bool>>) -> bool {
        set.iter().collect::<BitSet<BitSet<bool>>>() == set
    }
}
