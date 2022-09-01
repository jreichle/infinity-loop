use quickcheck::Arbitrary;

use super::{cardinality::Cardinality, finite::Finite};

/// Number in the range between `MIN` and `MAX` inclusive
///
/// Only constructable through the [`Arbitrary`] trait
///
/// # Example
///
/// ```rust
/// # use quickcheck_macros::quickcheck;
/// # use game::model::interval::Interval;
///
/// #[quickcheck]
/// fn a_property_test(number: Interval<2, 10>) -> bool {
///     /* omitted */
///     # true
/// }
/// ```
///
/// # Invariant
///
/// `∀min, max : usize. min <= max`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Interval<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> Interval<MIN, { MAX }> {
    /// Check for invariant at compile time
    const INVARIANT: () = if MIN > MAX {
        panic!("Interval invariant: 'MIN' must be smaller than 'MAX'")
    };

    /// Lowest possible value
    pub const MIN: usize = MIN;

    /// Highest possible value
    pub const MAX: usize = MAX;

    /// Converts to [`i32`] unchecked
    pub fn to_i32(self) -> i32 {
        self.0 as i32
    }

    /// Converts to [`i64`] unchecked
    pub fn to_i64(self) -> i64 {
        self.0 as i64
    }

    /// Converts to [`isize`] unchecked
    pub fn to_isize(self) -> isize {
        self.0 as isize
    }

    /// Converts to [`u32`] unchecked
    pub fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts to [`u64`] unchecked
    pub fn to_u64(self) -> u64 {
        self.0 as u64
    }

    /// Converts to [`usize`]
    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize> Arbitrary for Interval<MIN, { MAX }> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Interval(usize::arbitrary(g) % (MAX - MIN + 1) + MIN)
    }
}

impl<const MIN: usize, const MAX: usize> Cardinality for Interval<MIN, { MAX }> {
    const CARDINALITY: u64 = (Self::MAX - Self::MIN) as u64 + 1;
}

impl<const MIN: usize, const MAX: usize> Finite for Interval<MIN, { MAX }> {
    fn unchecked_index_to_enum(value: u64) -> Self {
        Self(value as usize + Self::MIN)
    }

    fn enum_to_index(&self) -> u64 {
        (self.0 - Self::MIN) as u64
    }
}

/// Number in the range between `0` and `Max` inclusive
///
/// Only constructable through the [`Arbitrary`] trait
pub type Max<const MAX: usize> = Interval<0, MAX>;
