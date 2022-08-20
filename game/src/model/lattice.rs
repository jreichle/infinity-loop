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
    /// forms monoid with [BoundedLattice<A>::MAXIMUM] as neutral element
    fn meet(self, other: Self) -> Self;

    /// least upper bound of both elements
    ///
    /// forms monoid with [BoundedLattice<A>::MINIMUM] as neutral element
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

// meet       = and
// join       = or
// complement = not

trait MeetSemilattice: Sized + BitAnd<Output = Self> {}
trait JoinSemilattice: Sized + BitOr<Output = Self> {}
trait Lattice: JoinSemilattice + MeetSemilattice {}

impl<A: MeetSemilattice + JoinSemilattice> Lattice for A {}

trait BoundedLattice2: Lattice {
    /// least
    const BOTTOM: Self;

    /// greatest
    const TOP: Self;
}

trait DistributiveLattice: Not<Output = Self> {}

trait BooleanAlgebra: DistributiveLattice {}

impl<A: BoundedLattice2 + DistributiveLattice> BooleanAlgebra for A {}

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
inline BitArray in EnumSet
make solver to propagator by accepting evidence as possible tiles
put general purpose features into core folder
maybe seperate levelstream and unfold iterator

*/
