use std::ops::{BitAnd, BitOr, Not};

use super::{enumset::EnumSet, finite::Finite, tile::Tile};

///! operator relation of boolean algebra
///! | lattice    | operator                         | logic | set                | neutral element  | name    |
///! |------------|----------------------------------|-------|--------------------|------------------|---------|
///! | meet       | [`bitAnd`][std::ops::BitAnd] `&` | `∧`   | `∩` (intersection) | `1`              | `full`  |
///! | join       | [`bitOr`][std::ops::BitOr] `\|`  | `∨`   | `∪` (union)        | `0`              | `empty` |
///! | complement | [`not`][std::ops::Not] `!`       | `¬`   | `S^C` (complement) |                  |         |

/// Commutative, idempotent Semigroup "meet" = [`BitAnd`] over poset
pub trait MeetSemilattice: Sized + BitAnd<Output = Self> {}

/// Commutative, idempotent Semigroup "join" = [`BitOr`] over poset
pub trait JoinSemilattice: Sized + BitOr<Output = Self> {}

/// Commutative, idempotent Semigroups "meet" = [`BitAnd`] and "join" = [`BitOr`] over poset
pub trait Lattice: JoinSemilattice + MeetSemilattice {}

impl<A: MeetSemilattice + JoinSemilattice> Lattice for A {}

/// [Bounded Lattice](https://en.wikipedia.org/wiki/Lattice_(order)) is a Lattice with neutral elements for "meet" and "join"
pub trait BoundedLattice: Lattice {
    /// least element
    ///
    /// neutral element of the join monoid
    const BOTTOM: Self;

    /// greatest element
    ///
    /// neutral element of the meet monoid
    const TOP: Self;
}

pub trait BoundedLatticeExt<A>: IntoIterator<Item = A> + Sized
where
    A: BoundedLattice + PartialEq,
{
    /// greatest lower bound of all values
    ///
    /// lazy meet fold: early return if [`Self::BOTTOM`]
    fn and(self) -> A {
        let mut acc = A::TOP;
        for v in self.into_iter() {
            if acc == A::BOTTOM {
                break;
            }
            acc = acc & v
        }
        acc
    }

    /// least upper bound of all values
    ///
    /// lazy join fold: early return if [`Self::TOP`]
    fn or(self) -> A {
        let mut acc = A::BOTTOM;
        for v in self.into_iter() {
            if acc == A::TOP {
                break;
            }
            acc = acc | v
        }
        acc
    }
}

impl<I: IntoIterator<Item = A>, A: PartialEq + BoundedLattice> BoundedLatticeExt<A> for I {}

/// Lattice with distributive "meet" and "join"
pub trait DistributiveLattice: Lattice {}

/// A bounded and distributive lattice with an inverse for every element
pub trait BooleanAlgebra: BoundedLattice + DistributiveLattice + Not<Output = Self> {}

impl<A: BoundedLattice + DistributiveLattice + Not<Output = A>> BooleanAlgebra for A {}

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

impl BoundedLattice for bool {
    const BOTTOM: Self = false;

    const TOP: Self = true;
}

impl DistributiveLattice for bool {}

impl<A: Finite> JoinSemilattice for EnumSet<A> {}

impl<A: Finite> MeetSemilattice for EnumSet<A> {}

impl<A: Finite> BoundedLattice for EnumSet<A> {
    const BOTTOM: Self = Self::EMPTY;

    const TOP: Self = Self::FULL;
}

impl<A: Finite> DistributiveLattice for EnumSet<A> {}

impl<A: Finite> JoinSemilattice for Tile<A> {}

impl<A: Finite> MeetSemilattice for Tile<A> {}

impl<A: Finite> BoundedLattice for Tile<A> {
    const BOTTOM: Self = Self::NO_CONNECTIONS;

    const TOP: Self = Self::ALL_CONNECTIONS;
}

impl<A: Finite> DistributiveLattice for Tile<A> {}

#[cfg(test)]
mod tests {

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

    fn join_identity_element<A: BoundedLattice + PartialEq + Copy>(x: A) -> bool {
        x | A::BOTTOM == x
    }

    fn meet_identity_element<A: BoundedLattice + PartialEq + Copy>(x: A) -> bool {
        x & A::TOP == x
    }

    fn and_fold_is_lazy<A: BoundedLattice + PartialEq + Clone>(x: A) -> bool {
        iter::once(A::BOTTOM).chain(iter::repeat(x)).and() == A::BOTTOM
    }

    fn or_fold_is_lazy<A: BoundedLattice + PartialEq + Clone>(x: A) -> bool {
        iter::once(A::TOP).chain(iter::repeat(x)).or() == A::TOP
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
