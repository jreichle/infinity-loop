use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait Num
where
    Self: Sized
        + Copy
        + Clone
        + Add<Output = Self>
        + AddAssign
        + Mul<Output = Self>
        + MulAssign
        + BitOr<Output = Self>
        + BitOrAssign
        + BitAnd<Output = Self>
        + BitAndAssign
        + BitXor<Output = Self>
        + BitXorAssign
        + Shl<Output = Self>
        + ShlAssign
        + Shr<Output = Self>
        + ShrAssign,
{
    const ZERO: Self;

    const ONE: Self;

    const BITS: Self;

    const MIN: Self;

    const MAX: Self;
}

impl Num for u8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as u8;

    const MIN: Self = Self::MIN as u8;

    const MAX: Self = Self::MAX as u8;
}

impl Num for u16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as u16;

    const MIN: Self = Self::MIN as u16;

    const MAX: Self = Self::MAX as u16;
}

impl Num for u32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as u32;

    const MIN: Self = Self::MIN as u32;

    const MAX: Self = Self::MAX as u32;
}

impl Num for u64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as u64;

    const MIN: Self = Self::MIN as u64;

    const MAX: Self = Self::MAX as u64;
}

impl Num for u128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as u128;

    const MIN: Self = Self::MIN as u128;

    const MAX: Self = Self::MAX as u128;
}

impl Num for usize {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as usize;

    const MIN: Self = Self::MIN as usize;

    const MAX: Self = Self::MAX as usize;
}

impl Num for i8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as i8;

    const MIN: Self = Self::MIN as i8;

    const MAX: Self = Self::MAX as i8;
}

impl Num for i16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as i16;

    const MIN: Self = Self::MIN as i16;

    const MAX: Self = Self::MAX as i16;
}

impl Num for i32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as i32;

    const MIN: Self = Self::MIN as i32;

    const MAX: Self = Self::MAX as i32;
}

impl Num for i64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as i64;

    const MIN: Self = Self::MIN as i64;

    const MAX: Self = Self::MAX as i64;
}

impl Num for i128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as i128;

    const MIN: Self = Self::MIN as i128;

    const MAX: Self = Self::MAX as i128;
}

impl Num for isize {
    const ZERO: Self = 0;

    const ONE: Self = 1;

    const BITS: Self = Self::BITS as isize;

    const MIN: Self = Self::MIN as isize;

    const MAX: Self = Self::MAX as isize;
}

trait Signed: Num + Sub<Output = Self> + SubAssign {}

impl Signed for i8 {}

impl Signed for i16 {}

impl Signed for i32 {}

impl Signed for i64 {}

impl Signed for i128 {}

impl Signed for isize {}

trait Unsigned: Num {}

impl Unsigned for u8 {}

impl Unsigned for u16 {}

impl Unsigned for u32 {}

impl Unsigned for u64 {}

impl Unsigned for u128 {}

impl Unsigned for usize {}
