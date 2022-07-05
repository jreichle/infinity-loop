use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;
use std::{collections::HashSet, fmt::Display, hash::Hash};

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
/// backtracking approach
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

    fn adjust_inner<B, F: Fn(Grid<A>) -> Grid<B>>(self, transform: F) -> SentinelGrid<B> {
        SentinelGrid(transform(self.0))
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

/// systematic view on grid to facilitate construction and solving
type Helper<A> = SentinelGrid<BitSet<Tile<A>>>;

type Superposition<A> = BitSet<Tile<A>>;

fn all_tile_configurations<A: EnumSetType + Finite>(tile: Tile<A>) -> BitSet<Tile<A>> {
    // insert successively rotated tiles until encountering repeated initial tile
    iter_fix(
        (BitSet::<Tile<A>>::EMPTY, tile),
        |(s, t)| (s.inserted(*t), t.rotated_clockwise(1)),
        |x, y| x.0 == y.0,
    )
    .0
}

/// grid of superimposed tiles in different configurations
fn to_configuration_space<A: EnumSetType + Finite>(
    grid: &SentinelGrid<Tile<A>>,
) -> SentinelGrid<BitSet<Tile<A>>> {
    grid.map(all_tile_configurations)
}

fn unique<A: Finite + Copy>(grid: SentinelGrid<BitSet<A>>) -> Option<Grid<A>> {
    grid.extract_grid()
        .map(BitSet::unwrap_if_singleton)
        .sequence()
}

fn is_solvable<A>(grid: &SentinelGrid<HashSet<A>>) -> bool {
    grid.0.elements().iter().all(|s| !s.is_empty())
}

/// restricts superposition to only include tiles with specified connection and direction
fn restrict_tile<A: EnumSetType + Finite>(
    connection: Connection<A>,
    superposition: Superposition<A>,
) -> Superposition<A> {
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
fn overlaps<A: EnumSetType + Finite>(superposition: Superposition<A>) -> Vec<Connection<A>> {
    // empty superposition leads to propagation of Absent and Present hints simultaneously
    let mut connections = Vec::new();
    connections.extend(
        Tile::meet_all(superposition.clone().into_iter())
            .0
            .into_iter()
            .map(|d| Connection(d, Status::Present)),
    );
    connections.extend(
        Tile::join_all(superposition.into_iter())
            .0
            .complement()
            .into_iter()
            .map(|d| Connection(d, Status::Absent)),
    );
    connections
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

pub fn solve(grid: &Grid<Tile<Square>>) -> Vec<Grid<Tile<Square>>> {
    let helper = to_configuration_space(&with_sentinels(grid));
    let solved = minimize(helper);
    let r = unique(solved);
    r.into_iter().collect()
    // while let None = r {
    //     let coord = solved.0.coordinates().into_iter().max_by_key(|c| solved.0[c]).expect("Expected branch in puzzle, but it there were no superpositions");
    //     solved.0[coord].into_iter().map(|x| HashSet::from(x)).map(|s| solved.0.try_adjust_at(coord, |_| s)).map(minimize).collect()
    // }
}

fn minimize(grid: Helper<Square>) -> Helper<Square> {
    iter_fix(
        grid,
        |g| g.0.coordinates().into_iter().fold(g.clone(), step),
        SentinelGrid::eq,
    )
}

fn step(grid: Helper<Square>, index: Coordinate<isize>) -> Helper<Square> {
    grid.0
        .get(index)
        .map(Superposition::clone)
        .map(overlaps)
        .unwrap_or_default()
        .into_iter()
        .fold(grid, |acc, c| {
            SentinelGrid(
                acc.0
                    .try_adjust_at(neighbor(index, c.0), |s| restrict_tile(c.opposite(), s)),
            )
        })
}

struct SolutionIterator<A>(Vec<A>);

impl Iterator for SolutionIterator<Grid<Tile<Square>>> {
    type Item = Grid<Tile<Square>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::tile::{Square, Tile};

    #[quickcheck]
    fn tile_configurations_have_same_number_of_connections(tile: Tile<Square>) -> bool {
        let connections = tile.0.len();
        all_tile_configurations(tile)
            .into_iter()
            .all(|t| t.0.len() == connections)
    }

    #[quickcheck]
    fn restrict_tile_sanity_check() -> bool {
        let superposition = BitSet::from_iter(all_tile_configurations(Tile(Up | Right)));
        let connection = Connection(Right, Status::Present);
        restrict_tile(connection, superposition)
            == BitSet::from_iter([Tile(Up | Right), Tile(Right | Down)])
    }

    #[quickcheck]
    fn overlaps_sanity_check(superposition: Superposition<Square>) -> bool {
        println!(
            "{:?} -> {:?}",
            superposition
                .clone()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
            overlaps(superposition)
        );
        true
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

    #[quickcheck]
    fn tile_configurations(tile: Tile<Square>) -> bool {
        println!("{}", all_tile_configurations(tile));
        true
    }
}
