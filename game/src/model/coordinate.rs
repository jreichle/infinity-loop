use quickcheck::{Arbitrary, Gen};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{num::Num, representable::Representable};

/// Represents spacial position by defining values for row and column and offers basic arithmetic operators
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct Coordinate<A> {
    pub row: A,
    pub column: A,
}

// it is currently impossible to generically cast number literals in a constant expression outside of macros
impl<A: Num + Copy> Coordinate<A> {
    /// neutral element for [Coordinate] with regards to addition
    pub const ORIGIN: Self = Self::of(A::ZERO);
}

impl<A> Coordinate<A> {
    #[inline(always)]
    pub const fn new(row: A, column: A) -> Self {
        Coordinate { row, column }
    }

    #[inline(always)]
    pub fn map<B, F: Fn(A) -> B>(self, transform: F) -> Coordinate<B> {
        Coordinate::new(transform(self.row), transform(self.column))
    }

    /// Elementwise application of a binary function
    ///
    /// applicative liftA2 combinator
    #[inline(always)]
    pub fn combine<B, C, F>(self, other: Coordinate<B>, transform: F) -> Coordinate<C>
    where
        F: Fn(A, B) -> C,
    {
        Coordinate::new(
            transform(self.row, other.row),
            transform(self.column, other.column),
        )
    }

    #[inline(always)]
    pub fn flat_map<B, F: Fn(A) -> Coordinate<B>>(self, transform: F) -> Coordinate<B> {
        Coordinate::new(transform(self.row).row, transform(self.column).column)
    }

    /// swaps row and column
    #[inline(always)]
    pub fn swap(self) -> Self {
        Coordinate::new(self.column, self.row)
    }

    #[inline(always)]
    pub fn sum(self) -> A::Output
    where
        A: Add,
    {
        self.row + self.column
    }

    #[inline(always)]
    pub fn product(self) -> A::Output
    where
        A: Mul,
    {
        self.row * self.column
    }

    pub fn traverse<B, F: Fn(A) -> Option<B>>(self, f: F) -> Option<Coordinate<B>> {
        self.map(f).sequence()
    }
}

impl<A: Copy> Coordinate<A> {
    #[inline(always)]
    pub const fn of(value: A) -> Self {
        Self::new(value, value)
    }

    /// Converts the coordinate into a tuple
    // optimization: coerce as no-op, if same runtime representation
    #[inline(always)]
    pub fn to_tuple(&self) -> (A, A) {
        (self.row, self.column)
    }

    /// Converts the coordinate into an array of size 2
    // optimization: coerce as no-op, if same runtime representation
    #[inline(always)]
    pub fn to_array(&self) -> [A; 2] {
        [self.row, self.column]
    }
}

impl<A: Ord> Coordinate<A> {
    /// takes the elementwise min
    pub fn min(self, other: Self) -> Self {
        self.combine(other, Ord::min)
    }

    /// takes the elementwise max
    pub fn max(self, other: Self) -> Self {
        self.combine(other, Ord::max)
    }

    /// Fixes the coordinate between two other coordinates
    pub fn clamp(self, start: Self, end: Self) -> Self {
        self.max(start).min(end)
    }
}

// combine implementations after stabilizing the [`Try`] trait
impl<A> Coordinate<Option<A>> {
    /// converts a Coordinate<Option> into Option<Coordinate>
    pub fn sequence(self) -> Option<Coordinate<A>> {
        Some(Coordinate::new(self.row?, self.column?))
    }
}

impl<A, E> Coordinate<Result<A, E>> {
    pub fn sequence(self) -> Result<Coordinate<A>, E> {
        Ok(Coordinate::new(self.row?, self.column?))
    }
}

impl<A: Copy> From<A> for Coordinate<A> {
    fn from(value: A) -> Self {
        Self::new(value, value)
    }
}

impl<A> From<(A, A)> for Coordinate<A> {
    /// isomorphic to [`(A, A)`]
    fn from(tuple: (A, A)) -> Self {
        // possible optimization: coerce as no-op, if same runtime representation
        Self::new(tuple.0, tuple.1)
    }
}

impl<A> From<[A; 2]> for Coordinate<A> {
    /// isomorphic to [`\[A; 2\]`]
    fn from(array: [A; 2]) -> Self {
        // possible optimization: coerce as no-op, if same runtime representation
        let [x, y] = array;
        Self::new(x, y)
    }
}

// impl<A, F: Fn(bool) -> A> From<F> for Coordinate<A> {
//     /// As representable functor Coordinate is isomorphic to [`Fn(bool) -> A`]
//     fn from(f: F) -> Self {
//         Self::new(f(false), f(true))
//     }
// }

impl<A: Display> Display for Coordinate<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row: {:>4}, column: {:>4})", self.row, self.column)
    }
}

impl<A: Add<Output = B>, B> Add for Coordinate<A> {
    type Output = Coordinate<B>;

    fn add(self, other: Self) -> Self::Output {
        self.combine(other, Add::add)
    }
}

impl<A: Copy + Add<Output = B>, B> Add<A> for Coordinate<A> {
    type Output = Coordinate<B>;

    fn add(self, rhs: A) -> Self::Output {
        Coordinate::new(self.row + rhs, self.column + rhs)
    }
}

impl<A: AddAssign> AddAssign for Coordinate<A> {
    fn add_assign(&mut self, other: Self) {
        self.row += other.row;
        self.column += other.column;
    }
}

impl<A: Sub<Output = B>, B> Sub for Coordinate<A> {
    type Output = Coordinate<B>;

    fn sub(self, other: Self) -> Self::Output {
        self.combine(other, Sub::sub)
    }
}

impl<A: Copy + Sub<Output = B>, B> Sub<A> for Coordinate<A> {
    type Output = Coordinate<B>;

    fn sub(self, rhs: A) -> Self::Output {
        Coordinate::new(self.row - rhs, self.column - rhs)
    }
}

impl<A: SubAssign> SubAssign for Coordinate<A> {
    fn sub_assign(&mut self, other: Self) {
        self.row -= other.row;
        self.column -= other.column;
    }
}

impl<A: Mul<Output = B>, B> Mul for Coordinate<A> {
    type Output = Coordinate<B>;

    fn mul(self, other: Self) -> Self::Output {
        self.combine(other, Mul::mul)
    }
}

impl<A: Copy + Mul<Output = B>, B> Mul<A> for Coordinate<A> {
    type Output = Coordinate<B>;

    fn mul(self, rhs: A) -> Self::Output {
        Coordinate::new(self.row * rhs, self.column * rhs)
    }
}

impl<A: MulAssign> MulAssign for Coordinate<A> {
    fn mul_assign(&mut self, other: Self) {
        self.row *= other.row;
        self.column *= other.column;
    }
}

impl<A: Div<Output = B>, B> Div for Coordinate<A> {
    type Output = Coordinate<B>;

    fn div(self, other: Self) -> Self::Output {
        self.combine(other, Div::div)
    }
}

impl<A: Copy + Div<Output = B>, B> Div<A> for Coordinate<A> {
    type Output = Coordinate<B>;

    fn div(self, rhs: A) -> Self::Output {
        Coordinate::new(self.row / rhs, self.column / rhs)
    }
}

impl<A: DivAssign> DivAssign for Coordinate<A> {
    fn div_assign(&mut self, other: Self) {
        self.row /= other.row;
        self.column /= other.column;
    }
}

impl<A: Neg<Output = B>, B> Neg for Coordinate<A> {
    type Output = Coordinate<B>;

    fn neg(self) -> Self::Output {
        Coordinate::new(-self.row, -self.column)
    }
}

impl<A> IntoIterator for Coordinate<A> {
    type Item = A;

    type IntoIter = <[A; 2] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        [self.row, self.column].into_iter()
    }
}

// implementation: fill any remaining slots with default or always require two values to construct a coordinate?
impl<A: Default> FromIterator<A> for Coordinate<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Self::new(
            iter.next().unwrap_or_default(),
            iter.next().unwrap_or_default(),
        )
        // Coordinate::new(iter.next(), iter.next()).sequence().unwrap_or_default()
    }
}

impl<A: Arbitrary> Arbitrary for Coordinate<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(A::arbitrary(g), A::arbitrary(g))
    }
}

impl<A: Copy> Representable<A> for Coordinate<A> {
    type Index = bool;

    fn tabulate<F: Fn(Self::Index) -> A>(f: F) -> Self {
        Self::new(f(false), f(true))
    }

    fn index(&self, index: Self::Index) -> A {
        if index {
            self.column
        } else {
            self.row
        }
    }
}

// omit testing
