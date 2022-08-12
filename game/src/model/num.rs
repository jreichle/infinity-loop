use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub trait Num
where
    Self: Sized
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
}

impl Num for u8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for u16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for u32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for u64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for u128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for usize {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for i8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for i16 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for i32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for i64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for i128 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}

impl Num for isize {
    const ZERO: Self = 0;

    const ONE: Self = 1;
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
