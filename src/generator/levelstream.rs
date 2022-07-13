use std::iter::repeat;

use crate::model::{
    coordinate::Coordinate,
    fastgen::generate,
    grid::Grid,
    testlevel::{char_to_tile, parse_level, TEST_LEVELS},
    tile::{Square, Tile},
};

///! level generator is an infinite stream of functions from integer seed value to level
///! and is defined by an anamorphism

/// all relevant metadata about levels for generating them
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LevelProperty {
    pub dimension: Coordinate<usize>,
    // maybe some notion of difficulty
    // ...
}

/// stream anamorphism / unfold
/// infinite by design
#[derive(Copy, Clone, Debug)]
struct Unfold<S, A> {
    state: S,
    step: fn(S) -> (A, S),
}

impl<S: Default, A: Default> Default for Unfold<S, A> {
    fn default() -> Self {
        Self {
            state: S::default(),
            step: |s| (A::default(), s),
        }
    }
}

impl<S, A> Unfold<S, A> {
    pub fn new(state: S, step: fn(S) -> (A, S)) -> Self {
        Self { state, step }
    }
}

impl<S: Copy, A> Iterator for Unfold<S, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let (value, next_state) = (self.step)(self.state);
        self.state = next_state;
        Some(value)
    }
}

// do not change the carefully crafted signatures in the following functions!!!

/// generates 10 levels each of successively larger dimensions
fn generate_levels(
    property: LevelProperty,
) -> impl Iterator<Item = Box<dyn Fn(u64) -> Grid<Tile<Square>>>> {
    Unfold::new(property, |p| {
        let dimension = p.dimension;
        (
            move |seed| generate(dimension, seed),
            LevelProperty {
                dimension: dimension + Coordinate::of(1),
            },
        )
    })
    .flat_map(|l| repeat(l).take(10))
    .map(|x| Box::new(x) as Box<_>)
}

/// curried constant function
fn constant<A: Clone + 'static, B>(value: A) -> Box<dyn Fn(B) -> A> {
    Box::new(move |_| value.clone())
}

/// hardcoded levels ignore seed value
fn hardcoded_levels() -> impl Iterator<Item = Box<dyn Fn(u64) -> Grid<Tile<Square>>>> {
    TEST_LEVELS
        .into_iter()
        .map(|l| parse_level(l, char_to_tile).unwrap())
        .map(constant)
}

/// stream of hardcoded levels followed by randomly generated levels
pub fn level_stream(
    property: LevelProperty,
) -> impl Iterator<Item = impl Fn(u64) -> Grid<Tile<Square>>> {
    hardcoded_levels().chain(generate_levels(property))
}
