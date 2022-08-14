use quickcheck::Arbitrary;

/// Number in the range between `MIN` and `MAX` inclusive
/// 
/// Only constructable through the [`Arbitrary`] trait
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

    /// Converts to [`usize`] unchecked
    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize> Arbitrary for Interval<MIN, { MAX }> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Interval(usize::arbitrary(g) % (MAX - MIN + 1) + MIN)
    }
}

/// Number in the range between `0` and `Max` inclusive
///
/// Only constructable through the [`Arbitrary`] trait
pub type Max<const MAX: usize> = Interval<0, MAX>;
