use std::ops::{BitAnd, BitOr, Not};

use super::{enumset::EnumSet, finite::Finite, tile::Tile};

/// [Bounded Lattice](https://en.wikipedia.org/wiki/Lattice_(order)) consisting of 2 commutative monoids "meet" and "join" over a poset
pub trait BoundedLattice: Sized {
    /// least element of the set
    ///
    /// forms monoid with [BoundedLattice<A>::join] as combining operator
    fn bottom() -> Self;

    /// greatest element of the set
    ///
    /// forms monoid with [BoundedLattice<A>::meet] as combining operator
    fn top() -> Self;

    /// greatest lower bound of both elements
    ///
    /// forms monoid with [BoundedLattice<A>::top] as neutral element
    fn meet(self, other: Self) -> Self;

    /// least upper bound of both elements
    ///
    /// forms monoid with [BoundedLattice<A>::bottom] as neutral element
    fn join(self, other: Self) -> Self;

    /// greatest lower bound of all elements
    ///
    /// shortcut monoidal fold over several elements
    fn meet_all<I: IntoIterator<Item = Self>>(elements: I) -> Self {
        elements.into_iter().fold(Self::top(), Self::meet)
    }

    /// least upper bound of all elements
    ///
    /// shortcut monoidal fold over several elements
    fn join_all<I: IntoIterator<Item = Self>>(elements: I) -> Self {
        elements.into_iter().fold(Self::bottom(), Self::join)
    }
}

impl<A: Finite> BoundedLattice for Tile<A> {
    fn bottom() -> Tile<A> {
        Self::NO_CONNECTIONS
    }

    fn top() -> Tile<A> {
        Self::ALL_CONNECTIONS
    }

    fn meet(self, other: Self) -> Self {
        Self(self.0.intersection(other.0))
    }

    fn join(self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
}

impl BoundedLattice for () {
    fn bottom() {}

    fn top() {}

    fn meet(self, _: ()) {
        self
    }

    fn join(self, _: ()) {
        self
    }
}

impl BoundedLattice for bool {
    fn bottom() -> bool {
        false
    }

    fn top() -> bool {
        true
    }

    fn meet(self, other: bool) -> bool {
        self || other
    }

    fn join(self, other: bool) -> bool {
        self && other
    }
}

#[cfg(test)]
mod tests {

    use crate::model::tile::Square;

    use super::*;

    // legend
    // ----------
    // meet   = ∧
    // join   = ∨
    // bottom = 0
    // top    = 1
    // ----------

    /// x ∨ (y ∨ z) = (x ∨ y) ∨ z
    ///
    /// x ∧ (y ∧ z) = (x ∧ y) ∧ z
    #[quickcheck]
    fn associativity(x: Tile<Square>, y: Tile<Square>, z: Tile<Square>) -> bool {
        x.join(y.join(z)) == x.join(y).join(z) && x.meet(y.meet(z)) == x.meet(y).meet(z)
    }

    /// x ∨ y = y ∨ x
    ///
    /// x ∧ y = y ∧ x
    #[quickcheck]
    fn commutativity(x: Tile<Square>, y: Tile<Square>) -> bool {
        x.meet(y) == y.meet(x) && x.join(y) == y.join(x)
    }

    /// x ∨ x = x
    ///
    /// x ∧ x = x
    #[quickcheck]
    fn idempotency(x: Tile<Square>) -> bool {
        x.join(x) == x && x.meet(x) == x
    }

    /// x ∨ (x ∧ y) = x
    ///
    /// x ∧ (x ∨ y) = x
    #[quickcheck]
    fn absorption(x: Tile<Square>, y: Tile<Square>) -> bool {
        x.meet(x.join(y)) == x && x.join(x.meet(y)) == x
    }

    /// x ∨ 0 = x
    ///
    /// x ∧ 1 = x
    #[quickcheck]
    fn identity_element(x: Tile<Square>) -> bool {
        x.join(Tile::bottom()) == x && x.meet(Tile::top()) == x
    }
}

/// | lattice    | operator                       | logic | set              |   |       |
/// |------------|--------------------------------|-------|------------------|---|-------|
/// | meet       | [`bitAnd`][std::ops::BitAnd] & | ∧     | ∩ (intersection) | 1 | full  |
/// | join       | [`bitOr`][std::ops::BitOr] \|  | ∨     | ∪ (union)        | 0 | empty |
/// | complement | [`not`][std::ops::Not] !       | ¬     | S^C (complement) |   |       |
trait MeetSemilattice: Sized + BitAnd<Output = Self> {}

trait JoinSemilattice: Sized + BitOr<Output = Self> {}

trait Lattice: JoinSemilattice + MeetSemilattice {}

impl<A: MeetSemilattice + JoinSemilattice> Lattice for A {}

trait BoundedLattice2: Lattice {
    /// least element
    const BOTTOM: Self;

    /// greatest element
    const TOP: Self;

    /// greatest lower bound of all values
    ///
    /// lazy fold: early return if [`Self::BOTTOM`]
    fn and<I: IntoIterator<Item = Self>>(iter: I) -> Self
    where
        Self: PartialEq,
    {
        let mut acc = Self::TOP;
        for v in iter.into_iter() {
            if acc == Self::BOTTOM {
                break;
            }
            acc = acc & v
        }
        acc
    }

    /// least upper bound of all values
    ///
    /// lazy fold: early return if [`Self::TOP`]
    fn or<I: IntoIterator<Item = Self>>(iter: I) -> Self
    where
        Self: PartialEq,
    {
        let mut acc = Self::BOTTOM;
        for v in iter.into_iter() {
            if acc == Self::TOP {
                break;
            }
            acc = acc | v
        }
        acc
    }
}
trait DistributiveLattice: Lattice {}

trait BooleanAlgebra: BoundedLattice2 + DistributiveLattice + Not<Output = Self> {}

impl<A: BoundedLattice2 + DistributiveLattice + Not<Output = Self>> BooleanAlgebra for A {}

// () does not implement bit operators
// impl JoinSemilattice for () {}

// impl MeetSemilattice for () {}

// impl BoundedLattice2 for () {

//     const BOTTOM: Self = ();

//     const TOP: Self = ();
// }

// impl DistributiveLattice for () {}

// bool ≅ EnumSet<()>
impl JoinSemilattice for bool {}

impl MeetSemilattice for bool {}

impl BoundedLattice2 for bool {
    const BOTTOM: Self = false;

    const TOP: Self = true;
}

impl DistributiveLattice for bool {}

impl<A: Finite> JoinSemilattice for EnumSet<A> {}

impl<A: Finite> MeetSemilattice for EnumSet<A> {}

impl<A: Finite> BoundedLattice2 for EnumSet<A> {
    const BOTTOM: Self = Self::EMPTY;

    const TOP: Self = Self::FULL;
}

impl<A: Finite> DistributiveLattice for EnumSet<A> {}

impl<A: Finite> JoinSemilattice for Tile<A> {}

impl<A: Finite> MeetSemilattice for Tile<A> {}

impl<A: Finite> BoundedLattice2 for Tile<A> {
    const BOTTOM: Self = Self::NO_CONNECTIONS;

    const TOP: Self = Self::ALL_CONNECTIONS;
}

impl<A: Finite> DistributiveLattice for Tile<A> {}

/*
hint function
make solver to propagator by accepting evidence as possible tiles
put general purpose features into core folder
maybe seperate levelstream and unfold iterator

*/

#[cfg(test)]
mod tests2 {

    use crate::model::tile::Square;

    use super::*;
    use std::iter::{self};

    #[quickcheck]
    fn bool_associativity(x: bool, y: bool, z: bool) -> bool {
        join_associativity(x, y, z) && meet_associativity(x, y, z)
    }

    #[quickcheck]
    fn bool_commutativity(x: bool, y: bool) -> bool {
        join_commutativity(x, y) && meet_commutativity(x, y)
    }

    #[quickcheck]
    fn bool_idempotency(x: bool) -> bool {
        join_idempotency(x) && meet_idempotency(x)
    }

    #[quickcheck]
    fn bool_absorption(x: bool, y: bool) -> bool {
        join_absorption(x, y) && meet_absorption(x, y)
    }

    #[quickcheck]
    fn bool_identity_element(x: bool) -> bool {
        join_identity_element(x) && meet_identity_element(x)
    }

    #[quickcheck]
    fn bool_distributivity(x: bool, y: bool, z: bool) -> bool {
        join_distributivity(x, y, z) && meet_distributivity(x, y, z)
    }

    #[quickcheck]
    fn bool_complement(x: bool) -> bool {
        join_complement(x) && meet_complement(x)
    }

    #[quickcheck]
    fn tile_associativity(x: Tile<Square>, y: Tile<Square>, z: Tile<Square>) -> bool {
        join_associativity(x, y, z) && meet_associativity(x, y, z)
    }

    #[quickcheck]
    fn tile_commutativity(x: Tile<Square>, y: Tile<Square>) -> bool {
        join_commutativity(x, y) && meet_commutativity(x, y)
    }

    #[quickcheck]
    fn tile_idempotency(x: Tile<Square>) -> bool {
        join_idempotency(x) && meet_idempotency(x)
    }

    #[quickcheck]
    fn tile_absorption(x: Tile<Square>, y: Tile<Square>) -> bool {
        join_absorption(x, y) && meet_absorption(x, y)
    }

    #[quickcheck]
    fn tile_identity_element(x: Tile<Square>) -> bool {
        join_identity_element(x) && meet_identity_element(x)
    }

    #[quickcheck]
    fn tile_distributivity(x: Tile<Square>, y: Tile<Square>, z: Tile<Square>) -> bool {
        join_distributivity(x, y, z) && meet_distributivity(x, y, z)
    }

    #[quickcheck]
    fn tile_complement(x: Tile<Square>) -> bool {
        join_complement(x) && meet_complement(x)
    }

    #[quickcheck]
    fn tile_fold_is_lazy(x: Tile<Square>) -> bool {
        and_fold_is_lazy(x) && or_fold_is_lazy(x)
    }

    fn join_associativity<A: JoinSemilattice + PartialEq + Copy>(x: A, y: A, z: A) -> bool {
        (x | y) | z == x | (y | z)
    }

    fn meet_associativity<A: MeetSemilattice + PartialEq + Copy>(x: A, y: A, z: A) -> bool {
        (x & y) & z == x & (y & z)
    }

    fn join_commutativity<A: JoinSemilattice + PartialEq + Copy>(x: A, y: A) -> bool {
        x | y == y | x
    }

    fn meet_commutativity<A: MeetSemilattice + PartialEq + Copy>(x: A, y: A) -> bool {
        x & y == y & x
    }

    fn join_idempotency<A: JoinSemilattice + PartialEq + Copy>(x: A) -> bool {
        x | x == x
    }

    fn meet_idempotency<A: MeetSemilattice + PartialEq + Copy>(x: A) -> bool {
        x & x == x
    }

    fn join_absorption<A: Lattice + PartialEq + Copy>(x: A, y: A) -> bool {
        x | (x & y) == x
    }

    fn meet_absorption<A: Lattice + PartialEq + Copy>(x: A, y: A) -> bool {
        x & (x | y) == x
    }

    fn join_identity_element<A: BoundedLattice2 + PartialEq + Copy>(x: A) -> bool {
        x | A::BOTTOM == x
    }

    fn meet_identity_element<A: BoundedLattice2 + PartialEq + Copy>(x: A) -> bool {
        x & A::TOP == x
    }

    fn and_fold_is_lazy<A: BoundedLattice2 + PartialEq + Clone>(x: A) -> bool {
        BoundedLattice2::and(iter::once(A::BOTTOM).chain(iter::repeat(x))) == A::BOTTOM
    }

    fn or_fold_is_lazy<A: BoundedLattice2 + PartialEq + Clone>(x: A) -> bool {
        BoundedLattice2::or(iter::once(A::TOP).chain(iter::repeat(x))) == A::TOP
    }

    fn join_distributivity<A: DistributiveLattice + PartialEq + Copy>(x: A, y: A, z: A) -> bool {
        x | (y & z) == (x | y) & (x | z)
    }

    fn meet_distributivity<A: DistributiveLattice + PartialEq + Copy>(x: A, y: A, z: A) -> bool {
        x & (y | z) == (x & y) | (x & z)
    }

    fn join_complement<A: BooleanAlgebra + PartialEq + Copy>(x: A) -> bool {
        x | !x == A::TOP
    }

    fn meet_complement<A: BooleanAlgebra + PartialEq + Copy>(x: A) -> bool {
        x & !x == A::BOTTOM
    }
}
