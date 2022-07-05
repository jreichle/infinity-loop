use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use enumset::{EnumSet, EnumSetType};

use super::tile::{Square, Tile};

pub trait Cardinality: Sized {
    /// number of inhabitants in the type
    const CARDINALITY: u64;
}

/// replace with [!] type once it is stabilized
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Void {}

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
    const CARDINALITY: u64 = A::CARDINALITY * N as u64;
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

impl Cardinality for Square {
    const CARDINALITY: u64 = 4;
}

impl<A: Cardinality + EnumSetType> Cardinality for Tile<A> {
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}

impl<A: Cardinality + EnumSetType> Cardinality for EnumSet<A> {
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}
