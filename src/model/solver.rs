use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;
use std::{fmt::Display, hash::Hash};

use enumset::{EnumSet, EnumSetType};
use quickcheck::Arbitrary;

use super::{
    bitset::BitSet,
    coordinate::Coordinate,
    finite::Finite,
    grid::Grid,
    lattice::BoundedLattice,
    tile::{Square, Tile},
};

/// this file contains solver algorithms
///
/// fundamental musings:
///
/// fundemantal approach
///
/// transform level into configuration space by turning grid of tiles into grid of tile configurations
///
/// brute force approach
///
/// configuration space of grid assuming 4 configurations per tile and a r=10 x c=10 grid:
/// 4 ^ (10 * 10) = ~1.6e29
/// => brute force approach unfeasible, early pruning necessary
///
///
/// square tiles under rotational symmetry form 6 equivalence classes
///
/// backtracking approach: propagate connection constraints based on available tile configurations to neighboring tiles
///
/// constraints on tile configuration
///     * by symmetry tiles with no and all connections possess a singular configuration
///         => trivially solved
///     * neighbors of fixed tiles must follow the connection symmetry
///
///
///
///
/// general facts about solving
///
/// powerset of [Tile] forms a [bounded lattice](https://en.wikipedia.org/wiki/Lattice_(order)) partially ordered by inclusion with [EnumSet::all] as maximum and [EnumSet::new] as minimum
///
///

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SentinelGrid<A>(Grid<A>);

impl<A> SentinelGrid<A> {
    pub fn extract_grid(&self) -> Grid<A>
    where
        A: Clone,
    {
        Grid::init(self.0.rows() - 2, self.0.columns() - 2, |c| {
            self.0[c + Coordinate::of(1)].clone()
        })
    }

    fn map<B, F: Fn(A) -> B>(&self, transform: F) -> SentinelGrid<B>
    where
        A: Clone,
    {
        SentinelGrid(self.0.map(transform))
    }
}

impl Display for SentinelGrid<Superposition<Square>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.0.map(|s| Tile::join_all(s.into_iter()));
        Display::fmt(&grid, f)
    }
}

impl Arbitrary for SentinelGrid<Tile<Square>> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        with_sentinels(&Grid::arbitrary(g))
    }
}

/// function is specific to Square
fn with_sentinels(grid: &Grid<Tile<Square>>) -> SentinelGrid<Tile<Square>> {
    SentinelGrid(Grid::init(grid.rows() + 2, grid.columns() + 2, |c| {
        grid.get(c - Coordinate::of(1)).copied().unwrap_or_default()
    }))
}

fn coordinate_to_square(coordinate: Coordinate<isize>) -> Option<Square> {
    let row = coordinate.row;
    let column = coordinate.column;
    match (row, column) {
        (-1, 0) => Some(Up),
        (0, 1) => Some(Right),
        (1, 0) => Some(Down),
        (0, -1) => Some(Left),
        _ => None,
    }
}

fn square_to_coordinate(square: Square) -> Coordinate<isize> {
    match square {
        Up => Coordinate { row: -1, column: 0 },
        Right => Coordinate { row: 0, column: 1 },
        Down => Coordinate { row: 1, column: 0 },
        Left => Coordinate { row: 0, column: -1 },
    }
}

///
fn all_neighbors(index: Coordinate<isize>) -> Vec<Coordinate<isize>> {
    EnumSet::all().iter().map(|c| neighbor(index, c)).collect()
}

/// primitive operation linking [Coordinate<usize>] and [Square]
/// coordinate system exists outside of the grid
fn neighbor(index: Coordinate<isize>, direction: Square) -> Coordinate<isize> {
    index + square_to_coordinate(direction)
}

type Superposition<A> = BitSet<Tile<A>>;

/// systematic view on grid to facilitate construction and solving
type Sentinel<A> = SentinelGrid<Superposition<A>>;

/// witness for the ablility of [BitSet] to store at least Tile<Square>::CARDINALITY = 16 elements
const _: Superposition<Square> = BitSet::FULL;

/// expands tile to a the superposition of all element of the tiles equivalence class under rotational symmetry
///
/// examples:
/// * `[┗]` -> `{ [┗], [┏], [┓], [┛] }`
/// * `[╻]` -> `{ [╹], [╺], [╻], [╸] }`
/// * `[╋]` -> `{ [╋] }`
fn superimpose_tile<A: EnumSetType + Finite>(tile: Tile<A>) -> Superposition<A> {
    // insert successively rotated tiles until encountering repeated initial tile
    iter_fix(
        (BitSet::<Tile<A>>::EMPTY, tile),
        |(s, t)| (s.inserted(*t), t.rotated_clockwise(1)),
        |x, y| x.0 == y.0,
    )
    .0
}

/// grid of superimposed tiles in different configurations
fn superimpose_grid<A: EnumSetType + Finite>(grid: &SentinelGrid<Tile<A>>) -> Sentinel<A> {
    grid.map(superimpose_tile)
}

/// unwraps if all superpositions are collapsed (= only contain single state)
fn if_unique<A: Finite + Copy>(grid: &SentinelGrid<BitSet<A>>) -> Option<Grid<A>> {
    grid.extract_grid()
        .map(BitSet::unwrap_if_singleton)
        .sequence()
}

fn if_solvable<A>(grid: SentinelGrid<BitSet<A>>) -> Option<SentinelGrid<BitSet<A>>> {
    grid.0
        .elements()
        .iter()
        .all(|s| !s.is_empty())
        .then_some(grid)
}

/// restricts superposition to only include tiles with specified connection and direction
fn restrict_tile<A>(connection: Connection<A>, superposition: Superposition<A>) -> Superposition<A>
where
    A: EnumSetType + Finite,
{
    let iter = superposition.into_iter();
    match connection {
        Connection(ref d, Status::Absent) => iter.filter(|t| !t.0.contains(*d)).collect(),
        Connection(ref d, Status::Present) => iter.filter(|t| t.0.contains(*d)).collect(),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Status {
    Absent,
    Present,
}

impl Arbitrary for Status {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&[Status::Absent, Status::Present]).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Connection<A>(A, Status);

impl<A: Arbitrary> Arbitrary for Connection<A> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Connection(A::arbitrary(g), Status::arbitrary(g))
    }
}

impl Connection<Square> {
    fn opposite(&self) -> Self {
        match self {
            Connection(d, s) => Connection(d.opposite(), *s),
        }
    }
}

/// extract overlapping connections from superposition
///
/// eg. if there is a common [Up] connection between all superpositions, then the result includes [Connection::Present(Up)]
///
/// returned directions are guaranteed unique
fn extract_overlaps<A: EnumSetType + Finite>(
    superposition: Superposition<A>,
) -> Vec<Connection<A>> {
    // empty superposition leads to propagation of Absent and Present hints simultaneously
    let present_connections = Tile::meet_all(superposition)
        .0
        .into_iter()
        .map(|d| Connection(d, Status::Present));
    let absent_connections = Tile::join_all(superposition)
        .0
        .complement()
        .into_iter()
        .map(|d| Connection(d, Status::Absent));
    present_connections.chain(absent_connections).collect()
}

/// iterative fixed point of a function
///
/// applies function repeatedly starting from initial value until `condition(x, step(x))` holds true
fn iter_fix<A, F, T>(initial: A, step: F, condition: T) -> A
where
    F: Fn(&A) -> A,
    T: Fn(&A, &A) -> bool,
{
    let mut initial = initial;
    loop {
        let next = step(&initial);
        if condition(&initial, &next) {
            return initial;
        }
        initial = next;
    }
}

fn minimize(grid: Sentinel<Square>) -> Sentinel<Square> {
    iter_fix(
        grid,
        |g| g.0.coordinates().into_iter().fold(g.clone(), step),
        SentinelGrid::eq,
    )
}

/// chooses a tile with supplied heuristic and creates a new grid with the tile fixed to one state for each state in the superposition of that tile
fn branch<A: EnumSetType + Finite, F: Fn(&Sentinel<A>) -> Coordinate<isize>>(
    grid: &Sentinel<A>,
    heuristic: F,
) -> Vec<Sentinel<A>> {
    let coordinate = heuristic(grid);
    grid.0
        .get(coordinate)
        .copied()
        .unwrap_or_default()
        .iter()
        .map(|t| SentinelGrid(grid.0.try_adjust_at(coordinate, |_| BitSet::singleton(t))))
        .collect()
}

fn step(grid: Sentinel<Square>, index: Coordinate<isize>) -> Sentinel<Square> {
    grid.0
        .get(index)
        .map(Superposition::clone)
        .map(extract_overlaps)
        .unwrap_or_default()
        .into_iter()
        .fold(grid, |acc, c| {
            SentinelGrid(
                acc.0
                    .try_adjust_at(neighbor(index, c.0), |s| restrict_tile(c.opposite(), s)),
            )
        })
}

// hide concrete iterator implementation
pub fn solve(grid: &Grid<Tile<Square>>) -> impl Iterator<Item = Grid<Tile<Square>>> {
    SolutionIterator(vec![superimpose_grid(&with_sentinels(grid))])
}

/// lazy generation of solutions to unify API for quarying single and multiple solutions
///
/// stores a stack of solution candidates, which are successively refined
struct SolutionIterator<A>(Vec<A>);

impl Iterator for SolutionIterator<Sentinel<Square>> {
    type Item = Grid<Tile<Square>>;

    /// algorithm: backtracking with explicit stack
    ///
    /// pops solutions candidate from stack if available and attempts to solve it
    ///
    ///     1. contradiction: pop next element from stack and repeat procedure
    ///     2. unique solution: return in solved state
    ///     3. branching: select 1 candidate and repeat procedure, push rest on stack
    ///
    /// it is yet to be determined if a contradiction after branching can actually occur
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(candidate) = self.0.pop() {
            let minimized = minimize(candidate);
            match if_unique(&minimized) {
                None => {
                    // distinguish between no and several solutions
                    if_solvable(minimized).into_iter().for_each(|g| {
                        // current heuristic: pick superpositions with most states
                        let branch_candidates = branch(&g, |grid| {
                            grid.0
                                .coordinates()
                                .into_iter()
                                .max_by_key(|c| grid.0[*c].len())
                                .expect("Logical error: attempted to branch, but was unable")
                        });
                        self.0.extend(branch_candidates);
                    });
                }
                Some(g) => return Some(g),
            }
        }
        None
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::tile::{Square, Tile};

    #[quickcheck]
    fn tile_configurations_have_same_number_of_connections(tile: Tile<Square>) -> bool {
        let connections = tile.0.len();
        superimpose_tile(tile)
            .into_iter()
            .all(|t| t.0.len() == connections)
    }

    #[quickcheck]
    fn restrict_tile_sanity_check() -> bool {
        let superposition = BitSet::from_iter(superimpose_tile(Tile(Up | Right)));
        let connection = Connection(Right, Status::Present);
        restrict_tile(connection, superposition)
            == BitSet::from_iter([Tile(Up | Right), Tile(Right | Down)])
    }

    #[quickcheck]
    fn neighborhood_is_symmetric(index: Coordinate<i8>, direction: Square) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(|x| x as isize);
        let neighbor_index = neighbor(index, direction);
        index == neighbor(neighbor_index, direction.opposite())
    }

    /// successively visiting neighbors in each
    #[quickcheck]
    fn neighborhood_is_euclidian(index: Coordinate<i8>) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(|x| x as isize);
        EnumSet::all().into_iter().fold(index, neighbor) == index
    }
}
