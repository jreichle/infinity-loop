use super::{
    coordinate::Coordinate,
    enumset::EnumSet,
    grid::Grid,
    tile::{Square, Tile}, solver::*,
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
    let difference_grid = grid.zip(solution.with_index()).map(|((t, s), (i, c))| {
        if s == Status::Changed {
            if t == c {
                Ok(EnumSet::from(t))
            } else {
                Err(i) // differs from solution
            }
        } else {
            Ok(t.superimpose())
        }
    });

    match difference_grid.sequence() {
        Err(c) => Ok(c),
        Ok(g) => {
            // propagate information until a superposition collapses
            // concept: solve level with given information and generate return first newly collapsed index

            let mut sentinel = g.with_sentinels(Tile::NO_CONNECTIONS.into());
            let mut has_changed = true;
            // easier with a do while loop
            while has_changed {
                has_changed = false;
                for coordinate in sentinel.0.coordinates() {
                    let (s_new, changed) = propagate_restrictions_to_all_neighbors2(sentinel, coordinate, |old, new| {
                        if old != new {
                            has_changed = true;
                        }
                        old.len() != 1 && new.len() == 1
                    });
                    if let Some(c_changed) = changed.first() {
                        return Ok(*c_changed)
                    }
                    sentinel = s_new;
                }
            }
            Err("already solved".into())
        }
    }
    
}
