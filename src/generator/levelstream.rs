use std::iter::{repeat, FusedIterator};

use quickcheck::Arbitrary;

use crate::model::{
    coordinate::Coordinate,
    fastgen::generate,
    grid::Grid,
    testlevel::{char_to_tile, parse_level, TEST_LEVELS},
    tile::{Square, Tile},
};

///! level generator is an infinite stream of functions from integer seed value to level
///! and is defined by an anamorphism

/// all relevant metadata about levels in the context of level generation
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LevelProperty {
    pub dimension: Coordinate<usize>,
    // maybe some notion of difficulty
    // ...
}

impl Arbitrary for LevelProperty {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        const MAX: usize = 10;
        LevelProperty {
            dimension: Coordinate::<usize>::arbitrary(g).map(|v| v % MAX),
        }
    }
}

/// holds the information to generate successive values in an infinite iterator stream
///
/// represents canonical stream anamorphism / unfold with step function is the coalgebra of the stream type
///
/// resulting iterator is infinite by design
#[derive(Copy, Clone)]
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

impl<S: Clone, A> Iterator for Unfold<S, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        let (value, next_state) = (self.step)(self.state.clone());
        self.state = next_state;
        Some(value)
    }
}

impl<S: Clone, A> FusedIterator for Unfold<S, A> {}

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
                dimension: dimension + 1,
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

/// stream of the hardcoded levels followed by infinitely many randomly generated levels
pub fn level_stream(
    property: LevelProperty,
) -> impl Iterator<Item = impl Fn(u64) -> Grid<Tile<Square>>> {
    hardcoded_levels().chain(generate_levels(property))
}

/*
// unfold with mutable state
struct UnfoldMut<S, A> {
    state: S,
    step: fn(&mut S) -> A,
}

impl<S, A> UnfoldMut<S, A> {
    pub fn new(state: S, step: fn(&mut S) -> A) -> Self {
        Self { state, step }
    }
}

impl<S, A> Iterator for UnfoldMut<S, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.step)(&mut self.state))
    }
}

fn generate_levels2(
    property: LevelProperty,
) -> impl Iterator<Item = Box<dyn Fn(u64) -> Grid<Tile<Square>>>> {
    UnfoldMut::new(property, |p| {
        let dimension = p.dimension;
        p.dimension += Coordinate::of(1);
        move |seed| generate(dimension, seed)
    })
    .flat_map(|l| repeat(l).take(10))
    .map(|x| Box::new(x) as Box<_>)
}

struct Iterate<A> {
    state: A,
    step: fn(A) -> A,
}

struct IterateMut<A> {
    state: A,
    step: fn(&mut A),
}
*/

#[cfg(test)]
mod test {

    use super::*;

    #[quickcheck]
    fn level_stream_is_solvable(property: LevelProperty) -> bool {
        // hylomorphism
        generate_levels(property)
            .take(10)
            .enumerate()
            .all(|(i, f)| f(i as u64).solve().next().is_some())
    }
}
