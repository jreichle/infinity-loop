use std::{
    fmt::Display,
    hash::Hash,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitXor, Not, Shl, BitOrAssign, BitAndAssign},
};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite, lattice::BoundedLattice};

// prefered representation, supports sets with arbitrary capacity
// struct BitSet<A: Cardinality>([BitStorage; (A::CARDINALITY + CAPACITY - 1) / CAPACITY], PhantomData<A>);

/// Defines the runtime representation and storage [`CAPACITY`] of a [`BitSet`]
///
/// May be set to any unsigned integer type
type BitStorage = u64;

/// Indicates the maximum number of elements that can be stored in a [`BitSet`]
///
/// This is based on the number of bits in the underlying [`BitStorage`] type
const CAPACITY: u64 = BitStorage::BITS as u64;

/// Set for storing elements of statically enumerable types with known cardinality, as witnessed by the traits [Cardinality] and [Finite]
///
/// The capacity is determined at compile-time by the type of the stored elements and precludes the need for user management
///
/// This implementation uses a fixed amount of memory and does not grow dynamically
///
/// Using types that exceed the maximum storing capacity leads to a compile-time error: [`BitStorage::BITS`] ≥ [`A::CARDINALITY`]
///
/// # Invariants
///
/// 1. bits exceeding [`A::Cardinality`] are always set to 0
///
/// Invariant #1 ensures canonical representation for equality checks
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct BitSet<A>(BitStorage, PhantomData<A>);

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
        // delete last ", "-seperator
        string.pop();
        string.pop();
        write!(f, "{{{}}}", string)
    }
}

impl<A: Cardinality> Cardinality for BitSet<A> {
    // a BitSet with 64 elements works fine, except in that case calling CARDINALITY causes an overflow
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}

impl<A: Finite> Finite for BitSet<A> {
    fn index_to_enum(value: u64) -> Self {
        Self(value as BitStorage & Self::USED_BITS, PhantomData) // truncating
    }

    fn enum_to_index(&self) -> u64 {
        self.0 as u64
    }
}

impl<A> BitSet<A> {
    /// Set containing 0 elements
    pub const EMPTY: Self = Self(BitStorage::MIN, PhantomData);

    /// Indicates if the set contains 0 elements
    pub const fn is_empty(self) -> bool {
        self.0 == BitStorage::MIN
    }

    /// Returns number of elements in the set
    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns a set containing every element present in both sets
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0, PhantomData)
    }

    /// Returns a set containing any elements present in either set
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0, PhantomData)
    }

    /// Returns a set containing all elements in the first set without the elements in the second set
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0, PhantomData)
    }

    /// Returns a set containing all the elements that are contained in exactly one set
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0, PhantomData)
    }

    /// Indicates if the other set contains at least all elements of this one
    pub const fn is_subset(self, other: Self) -> bool {
        self.0 | other.0 == self.0
    }

    /// Indicates if this set contains at least all elements of the other one
    pub const fn is_superset(self, other: Self) -> bool {
        other.is_subset(self)
    }

    /// Indicates if both sets share no common elements
    pub const fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == BitStorage::MIN
    }
}

impl<A: Cardinality> BitSet<A> {
    /// Bitmask with the [`A:CARDINALITY`] least significant bits set to 1
    ///
    /// # Examples
    ///
    /// `BitSet::<bool>::USED_BITS == 0b0...0011`
    ///
    /// # Invariant
    ///
    /// `∀s : BitSet. s.0 & USED_BITS == s.0`
    ///
    /// or equivalently
    ///
    /// `∀s : BitSet. s.intersection(BitSet::FULL) == s`
    const USED_BITS: BitStorage = if CAPACITY >= A::CARDINALITY {
        // == `Self::CARDINALITY - 1` without risk of overflow
        BitStorage::MAX >> (CAPACITY - A::CARDINALITY) as BitStorage
    } else {
        panic!("BitSet only supports up to 64 elements")
    };

    /// Set containing all possible elements
    pub const FULL: Self = Self(Self::USED_BITS, PhantomData);

    /// Returns a set containing all elements not in this set
    ///
    /// Mutable variant of [`BitSet::not`]
    pub fn complement(mut self) {
        self.0 = !self.0 & Self::USED_BITS
    }
}

impl<A: Finite> BitSet<A> {
    /// Set only containing the given element
    pub fn singleton(element: A) -> Self {
        Self::EMPTY.inserted(element)
    }

    /// Checks if set contains a given element
    pub fn contains(self, element: A) -> bool {
        test_bit(self.0, element.enum_to_index() as BitStorage)
    }

    /// Inserts given element into the set
    ///
    /// Immutable variant of [`BitSet::insert`]
    pub fn inserted(self, element: A) -> Self {
        Self(
            set_bit(self.0, element.enum_to_index() as BitStorage),
            PhantomData,
        )
    }

    /// Removes given element from the set
    ///
    /// Immutable variant of [`BitSet::remove`]
    pub fn removed(self, element: A) -> Self {
        Self(
            clear_bit(self.0, element.enum_to_index() as BitStorage),
            PhantomData,
        )
    }

    /// Toggles given element in the set
    ///
    /// Immutable variant of [`BitSet::toggle`]
    pub fn toggled(self, element: A) -> Self {
        Self(
            toggle_bit(self.0, element.enum_to_index() as BitStorage),
            PhantomData,
        )
    }

    /// Inserts given element into the set and indicates if the set has changed
    ///
    /// Mutable variant of [`BitSet::inserted`]
    pub fn insert(mut self, element: A) -> bool {
        let old = self.0;
        self.0 = set_bit(self.0, element.enum_to_index() as BitStorage);
        self.0 != old
    }

    /// Removes given element from the set and indicates if the set has changed
    ///
    /// Mutable variant of [`BitSet::removed`]
    pub fn remove(mut self, element: A) -> bool {
        let old = self.0;
        self.0 = clear_bit(self.0, element.enum_to_index() as BitStorage);
        self.0 != old
    }

    /// Toggles given element in the set
    ///
    /// Mutable variant of [`BitSet::toggled`]
    pub fn toggle(mut self, element: A) {
        self.0 = toggle_bit(self.0, element.enum_to_index() as BitStorage);
    }

    /// Unwraps the only element of the set
    ///
    /// * returns `Some(e)` if set is a singleton
    /// * returns `None` if set contains several elements or is empty
    pub fn unwrap_if_singleton(self) -> Option<A> {
        if self.len() == 1 {
            Some(A::index_to_enum(self.0.trailing_zeros() as u64))
        } else {
            None
        }
    }

    /// Returns an iterator over the elements in the set
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

/// Iterator for [`BitSet`]
///
/// The Iterator implementation currently successively consumes the bits until reaching 0 and therefore cannot implement [`DoubleEndedIterator`]
pub struct Iter<A> {
    bits: BitStorage,
    index: BitStorage,
    phantom: PhantomData<A>,
}

impl<A: Finite> Iterator for Iter<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let trailing = self.bits.trailing_zeros() as BitStorage;

            self.bits >>= trailing;
            self.bits &= !1; // consume element at current index
            self.index += trailing;

            Some(A::index_to_enum(self.index as u64))
        }
    }
}

impl<A: Finite> ExactSizeIterator for Iter<A> {
    fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    // fn is_empty(&self) -> bool {
    //     self.bits == 0
    // }
}

impl<A: Finite> FusedIterator for Iter<A> {}

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

    /// Returns a set containing all elements not in this set
    ///
    /// Immutable variant of [`BitSet::complement`]
    fn not(self) -> Self::Output {
        Self(!self.0 & Self::USED_BITS, PhantomData)
    }
}

impl<A: Finite> BitOr for BitSet<A> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl<A: Finite> BitOrAssign for BitSet<A> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl<A: Finite> BitAnd for BitSet<A> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<A: Finite> BitAndAssign for BitSet<A> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
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
fn bit_at<A: Shl + From<u8>>(index: A) -> <A as Shl>::Output {
    A::from(1) << index
}

#[inline(always)]
fn test_bit<A>(value: A, index: A) -> bool
where
    A: Shl + Shl<Output = A> + BitAnd + BitAnd<Output = A> + PartialEq + From<u8>,
{
    value & bit_at(index) != A::from(0)
}

#[inline(always)]
fn set_bit<A>(value: A, index: A) -> <A as BitOr>::Output
where
    A: Shl + Shl<Output = A> + BitOr + From<u8>,
{
    value | bit_at(index)
}

#[inline(always)]
fn clear_bit<A>(value: A, index: A) -> <A as BitAnd>::Output
where
    A: Shl + Shl<Output = A> + BitAnd + Not<Output = A> + From<u8>,
{
    value & !bit_at(index)
}

#[inline(always)]
fn toggle_bit<A>(value: A, index: A) -> <A as BitXor>::Output
where
    A: Shl + Shl<Output = A> + BitXor + From<u8>,
{
    value ^ bit_at(index)
}

/// associate arbitrary type which has the same number of inhabitants as there are bits in Self: Self::BITS == Inhabitants::CARDINALITY
trait UsedBits {
    type Inhabitants: Cardinality;
}

impl UsedBits for u8 {
    type Inhabitants = BitSet<Option<bool>>;
}

impl UsedBits for u16 {
    type Inhabitants = BitSet<(bool, bool)>;
}

impl UsedBits for u32 {
    type Inhabitants = BitSet<Option<(bool, bool)>>;
}

impl UsedBits for u64 {
    type Inhabitants = fn(Option<bool>) -> (bool, bool); // BitSet<(bool, Option<bool>)>;
}

impl UsedBits for u128 {
    type Inhabitants = BitSet<Option<(bool, Option<bool>)>>;
}

/// compile-time proof that [`BitSet`] can store [`BitSetSize::BITS`] elements
#[allow(clippy::assertions_on_constants)]
const _: () = {
    type BitSetSizeInhabitants = <BitStorage as UsedBits>::Inhabitants;
    assert!(BitSetSizeInhabitants::CARDINALITY == CAPACITY);
    assert!(BitSet::<BitSetSizeInhabitants>::USED_BITS == BitStorage::MAX)
};

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
