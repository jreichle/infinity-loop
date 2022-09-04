use std::{
    cmp::Ordering,
    fmt::Display,
    hash::Hash,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Sub, SubAssign},
};

use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite};

// prefered representation, supports sets with arbitrary capacity
// struct EnumSet<A: Cardinality>([BitArray; (A::CARDINALITY + CAPACITY - 1) / CAPACITY], PhantomData<A>);

/// Defines the runtime representation and storage [`CAPACITY`] of a [`EnumSet`]
///
/// May be set to any unsigned integer type
type BitArray = u64;

/// Indicates the maximum number of elements that can be stored in a [`EnumSet`]
///
/// This is based on the number of bits in the underlying [`BitArray`] type
const CAPACITY: u64 = BitArray::BITS as u64;

/// Set data structure for storing elements of statically enumerable types with known cardinality, as witnessed by the traits [Cardinality] and [Finite]
///
/// The capacity is determined at compile-time by the type of the stored elements and precludes the need for user management
///
/// This implementation preallocates a fixed amount of memory and does not grow dynamically
///
/// Using types that exceed the maximum storing capacity leads to a compile-time error: [`BitArray::BITS`] ≥ [`A::CARDINALITY`](Cardinality::CARDINALITY)
///
/// This struct deliberately does not implement the [`Default`] trait, instead use [`EnumSet::EMPTY`] or [`EnumSet::FULL`]
///
/// `EnumSet` is a specialized implementation of an [`EnumMap`](super::enummap::EnumMap): `EnumSet<A> ≅ EnumMap<A, ()>`
///
/// # Iterator
///
/// [`EnumSet`] is its own [`Iterator`] representation
///
/// the successive values in the iterator are in strictly ascending order iff the EnumSet is order isomorphic
///
/// # Invariants
///
/// 1. bits exceeding [`Cardinality`] of `A` are always set to 0
/// 2. [`Finite`] of EnumSet<A> is order isomorphic ⟺ [`Finite`] of A is order isomorphic:
///     `∀x, y : A, s1, s2 : EnumSet<A>. (s1 ≤ s2 ⟺ s1.enum_to_index() ≤ s2.enum_to_index()) ⟺ (x ≤ y ⟺ x.enum_to_index() ≤ y.enum_to_index())`
///
/// Invariant #1 ensures canonical representation for easy equality checks
#[derive(Debug)]
pub struct EnumSet<A>(BitArray, PhantomData<A>); // alternative names: FiniteSet, FinSet

// most derivable traits are independent of type [`A`]
impl<A> Copy for EnumSet<A> {}

impl<A> Clone for EnumSet<A> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<A> PartialEq for EnumSet<A> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<A> Eq for EnumSet<A> {}

impl<A> PartialOrd for EnumSet<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<A> Ord for EnumSet<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<A> Hash for EnumSet<A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<A: Display + Finite> Display for EnumSet<A> {
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

impl<A: Cardinality> Cardinality for EnumSet<A> {
    // a EnumSet with 64 elements works fine, except in that case calling CARDINALITY causes an overflow
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}

impl<A: Finite> Finite for EnumSet<A> {
    fn unchecked_index_to_enum(value: u64) -> Self {
        Self(value as BitArray & Self::USED_BITS, PhantomData) // truncating
    }

    fn enum_to_index(&self) -> u64 {
        self.0 as u64
    }
}

impl<A> EnumSet<A> {
    /// Set containing 0 elements
    ///
    /// neutral element of the [`EnumSet::union`] monoid
    pub const EMPTY: Self = Self(BitArray::MIN, PhantomData);

    /// Indicates if the set contains 0 elements
    pub const fn is_empty(self) -> bool {
        self.0 == BitArray::MIN
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
        self.0 | other.0 == other.0
    }

    /// Indicates if this set contains at least all elements of the other one
    pub const fn is_superset(self, other: Self) -> bool {
        other.is_subset(self)
    }

    /// Indicates if both sets share no common elements
    pub const fn is_disjoint(self, other: Self) -> bool {
        self.0 & other.0 == BitArray::MIN
    }
}

impl<A: Cardinality> EnumSet<A> {
    /// Bitmask with the [`A:CARDINALITY`] least significant bits set to 1
    ///
    /// # Examples
    ///
    /// `EnumSet::<bool>::USED_BITS == 0b0...0011`
    ///
    /// # Invariant
    ///
    /// `∀s : EnumSet. s.0 & USED_BITS == s.0`
    ///
    /// or equivalently
    ///
    /// `∀s : EnumSet. s.intersection(EnumSet::FULL) == s`
    const USED_BITS: BitArray = if CAPACITY >= A::CARDINALITY {
        if A::CARDINALITY == 0 {
            0
        } else {
            // == `Self::CARDINALITY - 1` without risk of overflow
            BitArray::MAX >> (CAPACITY - A::CARDINALITY) as BitArray
        }
    } else {
        panic!("EnumSet only supports up to 64 elements")
    };

    /// Set containing all possible elements
    ///
    /// neutral element of the [`EnumSet::intersection`] monoid
    pub const FULL: Self = Self(Self::USED_BITS, PhantomData);

    /// Returns a set containing all elements not in this set
    ///
    /// Mutable variant of [`EnumSet::not`]
    pub fn complement(&mut self) {
        self.0 = !self.0 & Self::USED_BITS
    }
}

impl<A: Finite> EnumSet<A> {
    /// Checks if the set contains a given element
    pub fn contains(self, element: A) -> bool {
        test_bit(self.0, element.enum_to_index() as BitArray)
    }

    /// Inserts given element into the set
    ///
    /// Immutable variant of [`EnumSet::insert`]
    pub fn inserted(self, element: A) -> Self {
        Self(
            set_bit(self.0, element.enum_to_index() as BitArray),
            PhantomData,
        )
    }

    /// Removes given element from the set
    ///
    /// Immutable variant of [`EnumSet::remove`]
    pub fn removed(self, element: A) -> Self {
        Self(
            clear_bit(self.0, element.enum_to_index() as BitArray),
            PhantomData,
        )
    }

    /// Toggles given element in the set
    ///
    /// Immutable variant of [`EnumSet::toggle`]
    pub fn toggled(self, element: A) -> Self {
        Self(
            toggle_bit(self.0, element.enum_to_index() as BitArray),
            PhantomData,
        )
    }

    /// Inserts given element into the set and indicates if the set has changed
    ///
    /// Mutable variant of [`EnumSet::inserted`]
    pub fn insert(&mut self, element: A) -> bool {
        let old = self.0;
        self.0 = set_bit(self.0, element.enum_to_index() as BitArray);
        self.0 != old
    }

    /// Removes given element from the set and indicates if the set has changed
    ///
    /// Mutable variant of [`EnumSet::removed`]
    pub fn remove(&mut self, element: A) -> bool {
        let old = self.0;
        self.0 = clear_bit(self.0, element.enum_to_index() as BitArray);
        self.0 != old
    }

    /// Toggles given element in the set
    ///
    /// Mutable variant of [`EnumSet::toggled`]
    pub fn toggle(&mut self, element: A) {
        self.0 = toggle_bit(self.0, element.enum_to_index() as BitArray);
    }

    /// Unwraps the only element of the set
    ///
    /// * returns `Some(e)` if set is a singleton
    /// * returns `None` if set contains several elements or is empty
    pub fn unwrap_if_singleton(self) -> Option<A> {
        if self.0.is_power_of_two() {
            Some(A::unchecked_index_to_enum(self.0.trailing_zeros() as u64))
        } else {
            None
        }
    }

    /// Returns an iterator over the elements in the set
    pub fn iter(&self) -> Self {
        *self
    }
}

impl<A: Finite> Iterator for EnumSet<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let bits = self.0;
        if bits == 0 {
            None
        } else {
            let index = bits.trailing_zeros();
            self.0 = bits & (bits - 1); // delete least significant set bit
            Some(A::unchecked_index_to_enum(index as u64))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.count_ones() as usize;
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    fn last(self) -> Option<Self::Item> {
        let bits = self.0;
        if bits == 0 {
            None
        } else {
            Some(A::unchecked_index_to_enum(
                (BitArray::BITS - bits.leading_zeros() - 1) as BitArray,
            ))
        }
    }

    // [`Finite`] of `A` is an order isomorphismus ⟹ [`Iterator::min`] ≡ [`Iterator::next`] and [`Iterator::max`] ≡ [`Iterator::last`]
    // improves asymptotic runtime behavior from linear O(n) to constant O(1)
}

impl<A: Finite> ExactSizeIterator for EnumSet<A> {
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    // fn is_empty(&self) -> bool {
    //     self.bits == 0
    // }
}

impl<A: Finite> FusedIterator for EnumSet<A> {}

impl<A: Finite> DoubleEndedIterator for EnumSet<A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let bits = self.0;
        if bits == 0 {
            None
        } else {
            let index = (BitArray::BITS - bits.leading_zeros() - 1) as u64;
            self.0 = clear_bit(bits, index);
            Some(A::unchecked_index_to_enum(index))
        }
    }
}

impl<A: Finite> FromIterator<A> for EnumSet<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter().fold(Self::EMPTY, Self::inserted)
    }
}

impl<A: Finite> Extend<A> for EnumSet<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter().for_each(|e| {
            self.insert(e);
        })
    }
}

impl<A: Cardinality> Not for EnumSet<A> {
    type Output = Self;

    /// Returns a set containing all elements not in this set
    ///
    /// Immutable variant of [`EnumSet::complement`]
    fn not(self) -> Self::Output {
        Self(!self.0 & Self::USED_BITS, PhantomData)
    }
}

impl<A: Finite> BitOr for EnumSet<A> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl<A: Finite> BitOrAssign for EnumSet<A> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl<A: Finite> BitAnd for EnumSet<A> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<A: Finite> BitAndAssign for EnumSet<A> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl<A: Finite> Sub for EnumSet<A> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl<A: Finite> SubAssign for EnumSet<A> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<A: Finite> From<A> for EnumSet<A> {
    /// Set only containing the given element
    fn from(element: A) -> Self {
        EnumSet(set_bit(0, element.enum_to_index()), PhantomData)
    }
}

impl<A: Arbitrary + Finite> Arbitrary for EnumSet<A> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Vec::arbitrary(g).into_iter().collect()
    }
}

#[inline(always)]
fn bit_at<A: Shl + From<u8>>(index: A) -> <A as Shl>::Output {
    A::from(1) << index
}

#[inline(always)]
fn test_bit<A>(value: A, index: A) -> bool
where
    A: Shl<Output = A> + BitAnd + BitAnd<Output = A> + PartialEq + From<u8>,
{
    value & bit_at(index) != A::from(0)
}

#[inline(always)]
fn set_bit<A>(value: A, index: A) -> <A as BitOr>::Output
where
    A: Shl<Output = A> + BitOr + From<u8>,
{
    value | bit_at(index)
}

#[inline(always)]
fn clear_bit<A>(value: A, index: A) -> <A as BitAnd>::Output
where
    A: Shl<Output = A> + BitAnd + Not<Output = A> + From<u8>,
{
    value & !bit_at(index)
}

#[inline(always)]
fn toggle_bit<A>(value: A, index: A) -> <A as BitXor>::Output
where
    A: Shl<Output = A> + BitXor + From<u8>,
{
    value ^ bit_at(index)
}

#[macro_export]
macro_rules! enumset {
    ( $( $e:expr ),* ) => {{
        #[allow(unused_mut)]
        let mut set = $crate::core::enumset::EnumSet::EMPTY;
        $(
            set.insert($e);
        )*
        set
    }};
}

/// associate arbitrary type which has the same number of inhabitants as there are bits in Self: Self::BITS == Inhabitants::CARDINALITY
trait UsedBits {
    type Inhabitants: Cardinality;
}

impl UsedBits for u8 {
    type Inhabitants = EnumSet<Option<bool>>;
}

impl UsedBits for u16 {
    type Inhabitants = EnumSet<(bool, bool)>;
}

impl UsedBits for u32 {
    type Inhabitants = EnumSet<Option<(bool, bool)>>;
}

impl UsedBits for u64 {
    type Inhabitants = fn(Option<bool>) -> (bool, bool); // EnumSet<(bool, Option<bool>)>;
}

impl UsedBits for u128 {
    type Inhabitants = EnumSet<Option<(bool, Option<bool>)>>;
}

/// compile-time proof that [`EnumSet`] can store [`EnumSetSize::BITS`] elements
#[allow(clippy::assertions_on_constants)]
const _: () = {
    type EnumSetSizeInhabitants = <BitArray as UsedBits>::Inhabitants;
    assert!(EnumSetSizeInhabitants::CARDINALITY == CAPACITY);
    assert!(EnumSet::<EnumSetSizeInhabitants>::USED_BITS == BitArray::MAX)
};

#[cfg(test)]
mod test {

    use std::collections::HashSet;

    use quickcheck::Gen;

    use crate::core::{enumset::*, finite::all_enums_ascending, interval::Max};
    use crate::model::tile::Square;

    #[quickcheck]
    fn set_to_index_and_back_is_id(set: EnumSet<EnumSet<Option<bool>>>) -> bool {
        set == EnumSet::unchecked_index_to_enum(set.enum_to_index())
    }

    #[quickcheck]
    fn index_to_set_and_back_is_id(value: u8) -> bool {
        type Set = EnumSet<EnumSet<Square>>;
        let value = value as u64 % Set::CARDINALITY;
        value == Set::unchecked_index_to_enum(value).enum_to_index()
    }

    #[quickcheck]
    fn bitshift_is_inverse_of_trailing_zeroes(index: Max<63>) -> bool {
        let index = index.to_u64();
        (1u64 << index).trailing_zeros() as u64 == index
    }

    /// not necessary, but desirable
    /// iff generic parameter is order isomorphic
    #[quickcheck]
    fn finite_impl_of_enumset_defines_order_isomorphism(
        s1: EnumSet<Square>,
        s2: EnumSet<Square>,
    ) -> bool {
        (s1 <= s2) == (s1.enum_to_index() <= s2.enum_to_index())
    }

    #[quickcheck]
    fn is_empty_iff_len_is_zero(set: EnumSet<EnumSet<bool>>) -> bool {
        set.is_empty() == (set.len() == 0)
    }

    #[quickcheck]
    fn insert_then_contains(mut set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.insert(element);
        set.contains(element)
    }

    #[quickcheck]
    fn insert_returns_inclusion(mut set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.contains(element) != set.insert(element)
    }

    #[quickcheck]
    fn remove_then_does_not_contain(
        mut set: EnumSet<EnumSet<bool>>,
        element: EnumSet<bool>,
    ) -> bool {
        set.remove(element);
        !set.contains(element)
    }

    #[quickcheck]
    fn remove_returns_inclusion(mut set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.contains(element) == set.remove(element)
    }

    #[quickcheck]
    fn toggle_flips_contains(mut set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        let contained = set.contains(element);
        set.toggle(element);
        contained != set.contains(element)
    }

    #[quickcheck]
    fn inserted_then_contains(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.inserted(element).contains(element)
    }

    #[quickcheck]
    fn removed_then_does_not_contain(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        !set.removed(element).contains(element)
    }

    #[quickcheck]
    fn toggled_flips_contains(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.contains(element) != set.toggled(element).contains(element)
    }

    #[quickcheck]
    fn singleton_then_unwrap_if_singleton_always_succeeds(element: EnumSet<bool>) -> bool {
        EnumSet::from(element).unwrap_if_singleton() == Some(element)
    }

    #[quickcheck]
    fn unwrap_if_singleton_is_some_iff_len_is_one(set: EnumSet<EnumSet<bool>>) -> bool {
        let len = set.len();
        // `Option::map_or` is the canonical fold
        set.unwrap_if_singleton().map_or(len != 1, |_| len == 1)
    }

    #[quickcheck]
    fn enumset_invariant_holds_true(set: EnumSet<EnumSet<bool>>) -> bool {
        set.0 & EnumSet::<EnumSet<bool>>::USED_BITS == set.0
            && set.intersection(EnumSet::FULL) == set
            && set.union(EnumSet::EMPTY) == set
    }

    #[quickcheck]
    fn set_and_not_set_is_disjoint(set: EnumSet<EnumSet<bool>>) -> bool {
        set.is_disjoint(!set)
    }

    #[quickcheck]
    fn iterator_of_singleton_set_contains_the_single_element(element: EnumSet<bool>) -> bool {
        let mut iter = EnumSet::from(element).iter();
        iter.next() == Some(element) && iter.next() == None
    }

    #[quickcheck]
    fn iter_then_collect_is_id(set: EnumSet<Max<{ CAPACITY as usize - 1 }>>) -> bool {
        set.iter().collect::<EnumSet<_>>() == set
    }

    #[quickcheck]
    fn iter_size_hint_is_exact(
        set: EnumSet<Max<{ CAPACITY as usize - 1 }>>,
        random: usize,
    ) -> bool {
        let skip_distance = random.checked_rem(set.len() as usize).unwrap_or_default();
        let len = set.iter().skip(skip_distance).collect::<Vec<_>>().len();
        set.iter().skip(skip_distance).size_hint() == (len, Some(len))
    }

    #[quickcheck]
    fn iter_count_is_correct(set: EnumSet<Max<{ CAPACITY as usize - 1 }>>, random: usize) -> bool {
        let skip_distance = random.checked_rem(set.len() as usize).unwrap_or_default();
        set.iter().skip(skip_distance).count()
            == set.iter().skip(skip_distance).collect::<Vec<_>>().len()
    }

    #[quickcheck]
    fn iter_last_is_last_value_of_iterator(
        set: EnumSet<Max<{ CAPACITY as usize - 1 }>>,
        random: usize,
    ) -> bool {
        let skip_distance = random.checked_rem(set.len() as usize).unwrap_or_default();
        set.iter().skip(skip_distance).collect::<Vec<_>>().last()
            == set.iter().skip(skip_distance).last().as_ref()
    }

    #[quickcheck]
    fn iterator_values_are_in_ascending_order(set: EnumSet<EnumSet<bool>>) -> bool {
        let iter = set.iter().map(|v| v.enum_to_index());
        iter.clone().zip(iter.skip(1)).all(|(x, y)| x < y)
    }

    #[quickcheck]
    fn rev_iter_then_collect_is_id(set: EnumSet<Max<{ CAPACITY as usize - 1 }>>) -> bool {
        set.iter().rev().collect::<EnumSet<_>>() == set
    }

    // test the invariant for all methods

    /// test all possible cases instead of randomly
    fn invariant<A: Finite>(operation: fn(EnumSet<A>, A) -> EnumSet<A>) {
        all_enums_ascending()
            .flat_map(|s| all_enums_ascending().map(move |e| operation(s, e)))
            .for_each(|s| assert_eq!(s.0 & EnumSet::<A>::USED_BITS, s.0));
    }

    #[test]
    fn ensure_invariant_inserted() {
        invariant::<EnumSet<bool>>(EnumSet::inserted);
    }
    #[test]
    fn ensure_invariant_removed() {
        invariant::<EnumSet<bool>>(EnumSet::removed);
    }

    #[test]
    fn ensure_invariant_toggled() {
        invariant::<EnumSet<bool>>(EnumSet::toggled);
    }

    #[quickcheck]
    fn inserted_is_idempotent(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.inserted(element) == set.inserted(element).inserted(element)
    }

    #[quickcheck]
    fn removed_is_idempotent(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set.removed(element) == set.removed(element).removed(element)
    }
    #[quickcheck]
    fn toggled_is_inverse_of_itself(set: EnumSet<EnumSet<bool>>, element: EnumSet<bool>) -> bool {
        set == set.toggled(element).toggled(element)
    }

    /// two components are interchangeable if all possible execution histories are identical (Liskov Substitution Principle)
    /// idea: generate random execution histories and compare all events (= outputs of functions) with reference implementation
    /// implementation is quite hacky and serves rather as proof of concept

    type ExecutionHistory = Vec<Command>;

    /// commands are defunctionalizations of EnumSet methods (initial encoding)
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    enum Command {
        Insert,
        Remove,
        // Toggled,
        Contains,
        Length,
        IsEmpty,

        Intersection,
        Union,
        Difference,
        SymmetricDifference,

        IsSubset,
        IsSuperSet,
        IsDisjoint,
    }

    impl Arbitrary for Command {
        fn arbitrary(g: &mut Gen) -> Self {
            *g.choose(&[
                Command::Insert,
                Command::Remove,
                Command::Contains,
                Command::Length,
                Command::IsEmpty,
                Command::Intersection,
                Command::Union,
                Command::Difference,
                Command::SymmetricDifference,
                Command::IsSubset,
                Command::IsSuperSet,
                Command::IsDisjoint,
            ])
            .unwrap()
        }
    }

    #[quickcheck]
    fn execution_history(history: ExecutionHistory) -> bool {
        let mut enum_set = EnumSet::<EnumSet<bool>>::EMPTY;
        let mut hash_set = HashSet::<EnumSet<bool>>::new();
        let mut g = Gen::new(100);
        history
            .into_iter()
            .for_each(move |c| relation(&mut enum_set, &mut hash_set, &mut g, c));
        true
    }

    fn relation<A: Finite + Copy + Eq + Hash + Arbitrary>(
        enum_set: &mut EnumSet<A>,
        hash_set: &mut HashSet<A>,
        g: &mut Gen,
        command: Command,
    ) {
        match command {
            Command::Insert => {
                let element = A::arbitrary(g);
                assert_eq!(enum_set.insert(element), hash_set.insert(element))
            }
            Command::Remove => {
                let element = A::arbitrary(g);
                assert_eq!(enum_set.remove(element), hash_set.remove(&element))
            }
            // Command::Toggled => {
            //     let element = A::arbitrary(g);
            //     hash_set.(element);
            //     (enum_set.toggled(element), hash_set)
            // },
            Command::Contains => {
                let element = A::arbitrary(g);
                assert_eq!(enum_set.contains(element), hash_set.contains(&element))
            }
            Command::Length => assert_eq!(enum_set.len() as usize, hash_set.len()),
            Command::IsEmpty => assert_eq!(EnumSet::<A>::is_empty(*enum_set), hash_set.is_empty()),
            Command::Intersection => {
                let set = EnumSet::<A>::arbitrary(g);
                *enum_set = enum_set.intersection(set);
                *hash_set = hash_set
                    .intersection(&set.into_iter().collect())
                    .cloned()
                    .collect::<HashSet<A>>()
            }
            Command::Union => {
                let set = EnumSet::<A>::arbitrary(g);
                *enum_set = enum_set.union(set);
                *hash_set = hash_set
                    .union(&set.into_iter().collect())
                    .cloned()
                    .collect::<HashSet<A>>()
            }
            Command::Difference => {
                let set = EnumSet::<A>::arbitrary(g);
                *enum_set = enum_set.difference(set);
                *hash_set = hash_set
                    .difference(&set.into_iter().collect())
                    .cloned()
                    .collect::<HashSet<A>>()
            }
            Command::SymmetricDifference => {
                let set = EnumSet::<A>::arbitrary(g);
                *enum_set = enum_set.symmetric_difference(set);
                *hash_set = hash_set
                    .symmetric_difference(&set.into_iter().collect())
                    .cloned()
                    .collect::<HashSet<A>>()
            }
            Command::IsSubset => {
                let set = EnumSet::<A>::arbitrary(g);
                assert_eq!(
                    enum_set.is_subset(set),
                    hash_set.is_subset(&set.into_iter().collect())
                )
            }
            Command::IsSuperSet => {
                let set = EnumSet::<A>::arbitrary(g);
                assert_eq!(
                    enum_set.is_superset(set),
                    hash_set.is_superset(&set.into_iter().collect())
                )
            }
            Command::IsDisjoint => {
                let set = EnumSet::<A>::arbitrary(g);
                assert_eq!(
                    enum_set.is_disjoint(set),
                    hash_set.is_disjoint(&set.into_iter().collect())
                )
            }
        }
    }
}

// Array of Struct <=> Struct of Array
// Vec<EnumSet<A>> == EnumSetVec<A>
//
// # Invariant
//
// s : EnumSetVec<A>. s.0.len() == A::CARDINALITY
// type Length = u64;
// const MAX_STORE: Length = Length::BITS as Length;
// struct EnumSetVec<A>(Vec<Length>, PhantomData<A>);
