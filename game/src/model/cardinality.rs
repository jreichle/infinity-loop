use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Defines the number of inhabitants of a type `|t|`
///
/// Implementations are restricted to types with finite number of elements ≤ [`u64::MAX`]
///
/// # Examples
///
/// ```
/// // bool = { false, true }
/// assert!(bool::CARDINALITY == 2);
///
/// // Maybe<bool> = { None, Some(false), Some(true) }
/// assert!(Maybe<bool>::CARDINALITY == 3);
///
/// assert!(Result<[bool; 5], Ordering>)::CARDINALITY == 35);
/// ```
pub trait Cardinality: Sized {
    /// number of inhabitants in the type
    const CARDINALITY: u64;
}

/// replace with [!] type once it is stabilized
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Void {}

impl<A: Cardinality> Cardinality for &A {
    const CARDINALITY: u64 = A::CARDINALITY;
}

impl Cardinality for Void {
    const CARDINALITY: u64 = 0;
}

impl Cardinality for () {
    const CARDINALITY: u64 = 1;
}

impl Cardinality for bool {
    const CARDINALITY: u64 = 2;
}

impl Cardinality for Ordering {
    const CARDINALITY: u64 = 3;
}

impl Cardinality for u8 {
    const CARDINALITY: u64 = u8::MAX as u64 + 1;
}

impl Cardinality for i8 {
    const CARDINALITY: u64 = u8::CARDINALITY;
}

impl Cardinality for u16 {
    const CARDINALITY: u64 = u16::MAX as u64 + 1;
}

impl Cardinality for i16 {
    const CARDINALITY: u64 = u16::CARDINALITY;
}

impl Cardinality for u32 {
    const CARDINALITY: u64 = u32::MAX as u64 + 1;
}

impl Cardinality for i32 {
    const CARDINALITY: u64 = u32::CARDINALITY;
}

impl Cardinality for char {
    const CARDINALITY: u64 = u32::CARDINALITY;
}

impl<A: Cardinality> Cardinality for Option<A> {
    const CARDINALITY: u64 = 1 + A::CARDINALITY;
}

impl<A: Cardinality, E: Cardinality> Cardinality for Result<A, E> {
    const CARDINALITY: u64 = A::CARDINALITY + E::CARDINALITY;
}

impl<A: Cardinality, B: Cardinality> Cardinality for (A, B) {
    const CARDINALITY: u64 = A::CARDINALITY * B::CARDINALITY;
}

impl<A: Cardinality, B: Cardinality, C: Cardinality> Cardinality for (A, B, C) {
    const CARDINALITY: u64 = A::CARDINALITY * B::CARDINALITY * C::CARDINALITY;
}

impl<A: Cardinality, const N: usize> Cardinality for [A; N] {
    const CARDINALITY: u64 = A::CARDINALITY.pow(N as u32);
}

impl<A: Cardinality, B: Cardinality> Cardinality for fn(A) -> B {
    const CARDINALITY: u64 = B::CARDINALITY.pow(A::CARDINALITY as u32);
}

impl<A: Cardinality, B: Cardinality, C: Cardinality> Cardinality for fn(A, B) -> C {
    const CARDINALITY: u64 = C::CARDINALITY.pow((A::CARDINALITY * B::CARDINALITY) as u32);
}

impl<K: Cardinality, V: Cardinality> Cardinality for HashMap<K, V> {
    const CARDINALITY: u64 = V::CARDINALITY.pow(K::CARDINALITY as u32);
}

impl<A: Cardinality> Cardinality for HashSet<A> {
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}
