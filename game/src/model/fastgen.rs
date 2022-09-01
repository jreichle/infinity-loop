use rand::{
    distributions::{Standard, Uniform},
    prelude::StdRng,
    Rng, SeedableRng,
};

use super::{
    coordinate::Coordinate,
    enumset::EnumSet,
    finite::Finite,
    grid::Grid,
    solver::*,
    tile::{Square, Tile},
};

///! since the tileset is not restricted and all possible tiles are available for constructing a level, any permutation of neighbor connections can be accomodated
///!
///! idea for a simplified wave function collapse algorithm:
///!     1. surround grid containing superpositions of all tiles with empty sentinel tiles
///!     2. propagate constraints of sentinel tiles
///!     3. collapse tiles randomly in a checkerboard pattern and propagate to neighbors
///!        tiles not in the checkerboard pattern are surrounded by collapsed tiles in all directions and are therefore collapsed by propagation
///!     4. delete sentinel tiles
///!
///! advantage: instead of alternately collapsing and propagating, first all are collapsed and then all are propagated
///
///! adjustment in implementation: instead of directly propagating collapsed superpositions, the whole grid is minimized at the end

impl<A: Finite> EnumSet<A> {
    /// Collapses superposition to a random single state if it is not empty
    fn collapse_random(self, random: usize) -> Self {
        random
            .checked_rem(self.len() as usize)
            .and_then(|i| self.into_iter().nth(i).map(EnumSet::from))
            .unwrap_or(self)
    }
}

/// Generates level deterministically
pub fn generate(dimension: Coordinate<usize>, seed: u64) -> Grid<Tile<Square>> {
    let minimized_grid = Grid::init(dimension, |_| EnumSet::FULL)
        .with_sentinels(Tile::NO_CONNECTIONS.into())
        .minimize();
    let grid = minimized_grid
        .0 // keep sentinel layer for
        .with_index()
        .zip(StdRng::seed_from_u64(seed).sample_iter(Standard))
        .map(|((c, e), r)| {
            if c.sum() % 2 == 0 {
                e.collapse_random(r)
            } else {
                e
            }
        });
    SentinelGrid(grid)
        .minimize()
        .extract_if_collapsed()
        .expect("error in algorithm")
}

impl<A: Finite> Grid<Tile<A>> {
    pub fn scramble(self, seed: u64) -> Self {
        let distribution = Uniform::new(0, A::CARDINALITY);
        self.zip(StdRng::seed_from_u64(seed).sample_iter(distribution))
            .map(|(t, r)| t.rotated_clockwise(r))
    }
}

#[cfg(test)]
mod test {

    use crate::model::interval::Max;

    use super::*;

    #[quickcheck]
    fn generated_levels_are_solvable(dimension: Coordinate<Max<20>>, seed: u64) -> bool {
        generate(dimension.map(Max::to_usize), seed)
            .solve()
            .next()
            .is_some()
    }
}
