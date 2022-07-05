use enumset::{EnumSet, EnumSetType};

use super::tile::Tile;

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
    fn meet_all<I: Iterator<Item = Self>>(elements: I) -> Self {
        elements.fold(Self::top(), Self::meet)
    }

    /// least upper bound of all elements
    ///
    /// shortcut monoidal fold over several elements
    fn join_all<I: Iterator<Item = Self>>(elements: I) -> Self {
        elements.fold(Self::bottom(), Self::join)
    }
}

impl<A: EnumSetType> BoundedLattice for Tile<A> {
    fn bottom() -> Tile<A> {
        Tile(EnumSet::empty())
    }

    fn top() -> Tile<A> {
        Tile(EnumSet::all())
    }

    fn meet(self, other: Self) -> Self {
        Tile(self.0.intersection(other.0))
    }

    fn join(self, other: Self) -> Self {
        Tile(self.0.union(other.0))
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
