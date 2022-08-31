use super::{
    coordinate::Coordinate,
    grid::Grid,
    solver::*,
    tile::{Square, Tile},
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Status {
    Changed,
    Original,
}

// introduce `Solvable` newtype, where the constructor ensures solvability of the puzzle
// Puzzle<A> -> Option<Solvable<A>>
// introduce `Unique` newtype, where the constructor ensures puzzle has unique solution
// Puzzle<A> -> Option<Unique<A>>

// else `Puzzle<S, A>`
// where S = Unsolvable | Unique | Multiple, corresponding to a list

// algorithm:
// 1. solve level with a trace of the collapsed superpositions in order
// 2. return first trace entry which is unequal to current configuration
/// **note:** currently only work fully for levels with a unique solution
pub fn generate_hint(grid: Grid<Tile<Square>>) -> Result<Coordinate<isize>, String> {
    // generate trace
    let sentinel = grid.with_sentinels(Tile::NO_CONNECTIONS).superimpose();

    let mut trace = vec![];
    iter_fix(
        sentinel,
        |s| {
            s.0.coordinates().into_iter().fold(s.clone(), |g, c| {
                let (s_new, v) = propagate_restrictions_to_all_neighbors2(g, c, |old, new| {
                    old.len() != 1 && new.len() == 1
                });
                trace.extend(v);
                s_new
            })
        },
        PartialEq::eq,
    );

    trace
        .into_iter()
        .find(|c| grid[*c].0.unwrap_if_singleton().is_some())
        .ok_or_else(|| "No hint available".into())
}
