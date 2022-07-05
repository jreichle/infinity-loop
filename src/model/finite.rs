use std::cmp::Ordering;

use super::cardinality::{Cardinality, Void};

/// witness for the bijection between enumeration and natural numbers
///
///
///
/// __laws__
/// * all indices are between `0` inclusive and `[Self::CARDINALITY]` exclusive
/// * `index_to_enum` ∘ `enum_to_index` == `identity` == `enum_to_index` ∘ `index_to_enum`
pub trait Finite: Cardinality {
    /// converts an integer index into the corresponding [Self]
    ///
    /// implementations may use modulo on the integer argument, since the domain is defined by the [Cardinality] trait
    fn index_to_enum(value: u64) -> Self;

    /// converts a [Self] into the corresponding integer index
    fn enum_to_index(&self) -> u64;

    fn successor(value: Self) -> Option<Self> {
        // untested for correct behavior around under- / overflow
        let next_index = value.enum_to_index() + 1;
        if next_index < Self::CARDINALITY {
            Some(Self::index_to_enum(next_index))
        } else {
            None
        }
    }

    fn predecessor(value: Self) -> Option<Self> {
        u64::checked_sub(value.enum_to_index(), 1).map(Self::index_to_enum)
    }

    /// end exclusive
    fn range(from: Self, to: Self) -> Vec<Self> {
        (from.enum_to_index()..to.enum_to_index())
            .map(Self::index_to_enum)
            .collect()
    }

    /// collection of all enum values
    ///
    /// implementation should return lazy iterator, but returning `impl <trait>` is disallowed in traits
    fn all_enums_ascending() -> Vec<Self> {
        (0..=Self::CARDINALITY - 1)
            .map(Self::index_to_enum)
            .collect()
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
