use super::{
    coordinate::Coordinate,
    grid::Grid,
    solver::*,
    tile::{Square, Tile},
};

// algorithm:
// 1. solve level with a trace of the collapsed superpositions in order
// 2. return first trace entry which is unequal to current configuration

/// Generates a trace of the successively solved tiles
///
/// can be memoized
pub fn generate_solving_trace(grid: &Grid<Tile<Square>>) -> Vec<(Coordinate<isize>, Tile<Square>)> {
    let mut stack = vec![(
        grid.with_sentinels(Tile::NO_CONNECTIONS).superimpose(),
        vec![],
    )];

    loop {
        let v = stack.pop();
        if v.is_none() {
            return vec![];
        }
        let (mut sentinel, mut trace) = v.unwrap();
        sentinel = iter_fix(
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
        if sentinel.extract_if_collapsed().is_some() {
            return trace;
        }

        // distinguish between no and several solutions
        if let Some(grid) = sentinel.clone().check_no_empty_superposition() {
            let c = most_superimposed_states(&grid);
            let v = grid.branch(most_superimposed_states).into_iter().map(|g| {
                let mut new_trace = trace.clone();
                new_trace.push((c - 1, g.0[c].unwrap_if_singleton().unwrap()));
                (g, new_trace)
            });
            stack.extend(v);
        }
    }
}

/// Returns hint based on given trace
pub fn get_hint(
    grid: &Grid<Tile<Square>>,
    trace: Vec<(Coordinate<isize>, Tile<Square>)>,
) -> Result<Coordinate<isize>, String> {
    trace
        .into_iter()
        .find(|(c, t)| grid[*c] != *t)
        .map(|(c, _)| c)
        .ok_or_else(|| "No hint available".into())
}

#[cfg(test)]
mod test {
    use crate::model::{
        coordinate::Coordinate,
        fastgen::generate,
        interval::{Interval, Max},
        tile::Tile,
    };

    use super::generate_solving_trace;

    #[quickcheck]
    fn number_of_hints(dimension: Coordinate<Max<20>>, seed: u64) -> bool {
        let grid = generate(dimension.map(Interval::to_usize), seed);
        let trace = generate_solving_trace(&grid);
        grid.elements()
            .into_iter()
            .filter(|t| *t != Tile::NO_CONNECTIONS && *t != Tile::ALL_CONNECTIONS)
            .count()
            == trace.len()
    }
}
