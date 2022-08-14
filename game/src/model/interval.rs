use quickcheck::Arbitrary;

/// Number in the range between `Min` and `Max` inclusive when constructed by the [`Arbitrary`] trait
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Interval<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> Interval<MIN, { MAX }> {
    const INVARIANT: () = if MIN > MAX {
        panic!("Min must be smaller than Max")
    };

    pub const MIN: usize = MIN;

    pub const MAX: usize = MAX;

    pub fn to_i32(self) -> i32 {
        self.0 as i32
    }

    pub fn to_i64(self) -> i64 {
        self.0 as i64
    }

    pub fn to_isize(self) -> isize {
        self.0 as isize
    }

    pub fn to_u32(self) -> u32 {
        self.0 as u32
    }

    pub fn to_u64(self) -> u64 {
        self.0 as u64
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl<const MIN: usize, const MAX: usize> Arbitrary for Interval<MIN, { MAX }> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Interval(usize::arbitrary(g) % (MAX - MIN + 1) + MIN)
    }
}

/// Number in the range between `0` and `Max` inclusive when constructed by the [`Arbitrary`] trait
pub type Max<const MAX: usize> = Interval<0, MAX>;
