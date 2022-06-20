use std::collections::HashSet;

use super::{grid::Grid, tile::{Tile, Square}};

/// this file contains solver algorithms
/// 
/// fundamental musings:
/// 
/// configuration space of grid assuming 4 configurations per tile and a r=10 x c=10 grid:
/// 4 ^ (10 * 10) = ~1.6e29
/// => brute force unfeasible

/// 0-tiles and 4-tiles are trivially solved
/// 3-tiles and I-2-tiles on the edges, L-2-tiles in the corners have only a single valid configuration
fn solve_trivial_tiles(grid: Grid<Tile<Square>>) -> Grid<HashSet<Tile<Square>>> {
    panic!()
}


/// 1. 0-tiles and 4-tiles are trivially solved and can be excluded
/// 2. 3-tiles and I-2-tiles on the edges, L-2-tiles in the corners have only a single valid configuration
/// 3. apply backtracking / determine tiles with least valid configurations / (some other heuristic)
fn solve(grid: Grid<HashSet<Tile<Square>>>) -> Grid<Tile<Square>> {
    panic!("to implement");

}

