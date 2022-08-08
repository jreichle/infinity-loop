use std::{cmp::Ordering, iter::successors};

use super::cardinality::{Cardinality, Void};

/// Witnesses the bijection between [`Self`] and the finite subset of
/// [natural numbers `ℕ`](https://en.wikipedia.org/wiki/Natural_number)
/// up to [`Self::CARDINALITY`] exclusive
///
/// With [`Cardinality`] as supertrait, implementations of [`Finite`] are
/// restricted to types with finite number of inhabitants ≤ [`u64::MAX`]
///
/// This requirement particularly excludes implementations for types with
/// dynamic sizes like lists, vectors and graphs
///
///
/// # Laws
///
/// * `x is Finite ⟺ ∃n. Fin n ≅ x`
/// * the isomorphism should additionally preserve the total order imposed by [`Ord`],
///     i.e. should be [order isomorphic](https://en.wikipedia.org/wiki/Order_isomorphism)
///     `∀x, y : Finite + Ord. x ≤ y ⟺ x.enum_to_index() ≤ y.enum_to_index()`
/// * all indices are between `0` inclusive and [`Self::CARDINALITY`] exclusive
/// * [`Finite::index_to_enum`] ∘ [`Finite::enum_to_index`]
///     ≡ [`identity`][std::convert::identity] ≡ [`Finite::enum_to_index`] ∘ [`Finite::index_to_enum`]
pub trait Finite: Cardinality {
    /// Converts an integer index into the corresponding [Self]
    ///
    /// The caller must ensure that this method is only called with values
    /// between 0 and [`Self::CARDINALITY`] exclusive
    ///
    /// Implementors may either
    ///
    /// 1. use the value modulo [`Self::CARDINALITY`]
    /// 2. panic, if value ≥ [`Self::CARDINALITY`]
    fn index_to_enum(value: u64) -> Self;

    /// Converts a [Self] into the corresponding natural number index
    fn enum_to_index(&self) -> u64;

    /// Returns the next element in the enumeration
    fn successor(&self) -> Option<Self> {
        let next_index = self.enum_to_index().checked_add(1)?;
        if next_index < Self::CARDINALITY {
            Some(Self::index_to_enum(next_index))
        } else {
            None
        }
    }

    /// Returns the previous element in the enumeration
    fn predecessor(&self) -> Option<Self> {
        self.enum_to_index().checked_sub(1).map(Self::index_to_enum)
    }

    /// Returns all values between two elements of the enumeration
    fn range(start: Self, end_exclusive: Self) -> Vec<Self> {
        (start.enum_to_index()..end_exclusive.enum_to_index())
            .map(Self::index_to_enum)
            .collect()
    }

    /// Returns all inhabitants in ascending order
    ///
    /// # Note
    ///
    /// Implementation should return lazy iterator, but returning `impl <trait>`
    /// is disallowed in traits as of Rust 1.6.3
    fn all_enums_ascending() -> Vec<Self> {
        (0..Self::CARDINALITY).map(Self::index_to_enum).collect()
    }
}

impl Finite for Void {
    fn index_to_enum(_: u64) -> Self {
        panic!()
    }

    fn enum_to_index(&self) -> u64 {
        panic!()
    }
}

impl Finite for () {
    fn index_to_enum(_: u64) {}

    fn enum_to_index(&self) -> u64 {
        0
    }
}

impl Finite for bool {
    fn index_to_enum(value: u64) -> Self {
        value != 0
    }

    fn enum_to_index(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
    }
}

impl Finite for Ordering {
    fn index_to_enum(value: u64) -> Self {
        match value % 3 {
            0 => Ordering::Less,
            1 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }

    fn enum_to_index(&self) -> u64 {
        match self {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        }
    }
}

impl<A: Finite> Finite for Option<A> {
    fn index_to_enum(value: u64) -> Self {
        match value {
            0 => None,
            n => Some(A::index_to_enum(n - 1)),
        }
    }

    fn enum_to_index(&self) -> u64 {
        match self {
            None => 0,
            Some(x) => x.enum_to_index() + 1,
        }
    }
}

impl<A: Finite, const N: usize> Finite for [A; N] {
    /// start with `[<0>, <0>, ..., <0>]` then count up starting from the first element
    fn index_to_enum(value: u64) -> Self {
        // std::array::from_fn(|i| A::index_to_enum((value / A::CARDINALITY.pow(i as u32)) % A::CARDINALITY))
        successors(Some(value), |x| Some(x / A::CARDINALITY))
            .take(N)
            .map(|x| A::index_to_enum(x % A::CARDINALITY))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("error: faulty implementation")) // statically known to be impossible
    }

    fn enum_to_index(&self) -> u64 {
        self.iter()
            .map(A::enum_to_index)
            .fold(0, |acc, x| acc * A::CARDINALITY + x)
    }
}
