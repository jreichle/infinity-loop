use std::iter::repeat;

use crate::model::{
    coordinate::Coordinate,
    grid::Grid,
    tile::{Square, Tile},
};

///! level generator is an infinite stream of functions from integer seed value to level
///! and is defined by an anamorphism

/// all relevant metadata about levels for generating them
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LevelProperty {
    dimension: Coordinate<usize>,
    // maybe some notion of difficulty
    // ...
}

/// infinite by design
#[derive(Copy, Clone, Debug)]
struct Unfold<S, A> {
    state: S,
    step: fn(S) -> (A, S),
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

/// generates 10 levels each of successively larger dimensions
fn generate_levels(property: LevelProperty) -> impl Iterator<Item = Grid<Tile<Square>>> {
    Unfold::new(property, |p| {
        (
            Grid::init(p.dimension.row, p.dimension.column, |_, _| {
                Tile(!!Square::Up)
            }),
            LevelProperty {
                dimension: p.dimension + Coordinate { row: 1, column: 1 },
            },
        )
    })
    .flat_map(|l| repeat(l).take(10))
}
