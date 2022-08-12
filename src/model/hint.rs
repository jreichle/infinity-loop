use super::{
    coordinate::Coordinate,
    enumset::EnumSet,
    grid::Grid,
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

/// algorithm
///     1. verify all changed grid tiles are correct by diffing with solution, else hint is incorrect tile
///     2. apply propagation algorithm to superposition grid with changed tiles collapsed and original tiles uncollapsed
fn generate_hint(grid: Grid<(Tile<Square>, Status)>) -> Result<Coordinate<isize>, String> {
    let solution = grid
        .map(|t| t.0)
        .solve()
        .next()
        .ok_or("puzzle has no solution")?;
    // how to handle multiple solutions? try to match any?
    let x = grid.zip(solution.with_index()).map(|((t, s), (i, c))| {
        if s == Status::Changed {
            if t == c {
                Ok(EnumSet::from(t))
            } else {
                Err(i)
            }
        } else {
            Ok(t.superimpose())
        }
    });

    // x.sequence();

    panic!()
}
