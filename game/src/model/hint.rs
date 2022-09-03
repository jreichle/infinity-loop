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
pub fn generate_hint(grid: &Grid<Tile<Square>>) -> Result<Coordinate<isize>, String> {
    let sentinel = grid.with_sentinels(Tile::NO_CONNECTIONS).superimpose();

    let mut trace = vec![];
    let solved = iter_fix(
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
        .map(|c| c - 1)
        .inspect(|c| log::info!("c: {c}"))
        .find(|c| grid[*c] != solved.0[*c + 1].unwrap_if_singleton().unwrap())
        .ok_or_else(|| "No hint available".into())
}

/// can be memoized
pub fn trace_solver(grid: &Grid<Tile<Square>>) -> Vec<(Coordinate<isize>, Tile<Square>)> {
    let sentinel = grid.with_sentinels(Tile::NO_CONNECTIONS).superimpose();

    let mut trace = vec![];
    iter_fix(
        sentinel,
        |s| {
            s.0.coordinates().into_iter().fold(s.clone(), |g, c| {
                let (s_new, v) = propagate_restrictions_to_all_neighbors2(g, c, |old, new| {
                    old.len() != 1 && new.len() == 1
                });
                trace.extend(
                    v.into_iter()
                        .map(|c| (c - 1, s_new.0[c].unwrap_if_singleton().unwrap())),
                ); // grid vs sentinelgrid indexing
                s_new
            })
        },
        PartialEq::eq,
    );
    trace
}

pub fn hint(
    grid: &Grid<Tile<Square>>,
    trace: Vec<(Coordinate<isize>, Tile<Square>)>,
) -> Result<Coordinate<isize>, String> {
    trace
        .into_iter()
        .inspect(|(c, _)| log::info!("c: {c}"))
        .find(|(c, t)| grid[*c] != *t)
        .map(|(c, _)| c)
        .ok_or_else(|| "No hint available".into())
}
