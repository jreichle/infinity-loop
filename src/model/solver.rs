use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;
use std::{collections::HashSet, hash::Hash, fmt::Display};

use enumset::{EnumSet, EnumSetType};

use super::{
    coordinate::Coordinate,
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
        let grid = self.0.map(|s| Tile::join_all(s.clone().into_iter()));
        Display::fmt(&grid, f)
    }
}

/// function is specific to Square
fn with_sentinels(grid: &Grid<Tile<Square>>) -> SentinelGrid<Tile<Square>> {
    SentinelGrid(Grid::init(grid.rows() + 2, grid.columns() + 2, |c| {
        grid.get(c - Coordinate::of(1))
            .map(|x| *x)
            .unwrap_or_default()
    }))
}

fn coordinate_to_square(coordinate: Coordinate<isize>) -> Option<Square> {
    let row = coordinate.row;
    let column = coordinate.column;
    match (row, column) {
        (0, 1) => Some(Up),
        (1, 0) => Some(Right),
        (0, -1) => Some(Down),
        (-1, 0) => Some(Left),
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

/// see [usize::checked_add_signed]
fn checked_add_signed(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs >= 0 {
        usize::checked_add(lhs, rhs as usize)
    } else {
        usize::checked_sub(lhs, rhs.unsigned_abs())
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
type Helper<A> = SentinelGrid<HashSet<Tile<A>>>;

type Superposition<A> = HashSet<Tile<A>>;

fn all_tile_configurations<A: EnumSetType + Hash>(tile: Tile<A>) -> HashSet<Tile<A>> {
    // insert successively rotated tiles until encountering repeated initial tile
    let mut tile = tile;
    let mut configurations = HashSet::new();
    while configurations.insert(tile) {
        tile = tile.rotated_clockwise(1);
    }
    configurations
}

/// grid of superimposed tiles in different configurations
fn to_configuration_space<A: EnumSetType + Hash>(
    grid: &SentinelGrid<Tile<A>>,
) -> SentinelGrid<HashSet<Tile<A>>> {
    grid.map(all_tile_configurations)
}

fn determine<A>(superposition: HashSet<A>) -> Option<A> {
    if superposition.len() == 1 {
        superposition.into_iter().next()
    } else {
        None
    }
}

fn unique<A: Clone>(grid: SentinelGrid<HashSet<A>>) -> Option<Grid<A>> {
    grid.extract_grid().map(determine).sequence()
}

fn is_solvable<A: EnumSetType>(grid: &SentinelGrid<HashSet<Tile<A>>>) -> bool {
    grid.0.elements().iter().all(|s| !s.is_empty())
}

/// restricts superposition to only include tiles with specified connection and direction
fn restrict_tile<A: EnumSetType + Hash>(
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Connection<A>(A, Status);

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
fn overlaps<A: EnumSetType>(superposition: Superposition<A>) -> Vec<Connection<A>> {
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
            .into_iter()
            .map(|d| Connection(d, Status::Absent)),
    );
    connections
}

/// iterative fixed point of a function
///
/// applies function to initial value until `condition(x, step(x))` holds true
fn iter_fix<A, F, T>(initial: A, step: F, condition: T) -> A
where
    A: Display,
    F: Fn(&A) -> A,
    T: Fn(&A, &A) -> bool,
{
    let mut initial = initial;
    loop {
        println!("{}", &initial);
        println!("-------");
        let next = step(&initial);
        if condition(&initial, &next) {
            return initial;
        }
        initial = next;
    }
}

pub fn solve(grid: &Grid<Tile<Square>>) -> Grid<Tile<Square>> {
    let helper = to_configuration_space(&with_sentinels(grid));
    let solved = iter_fix(
        helper,
        |g| g.0.coordinates().into_iter().fold(g.clone(), step),
        SentinelGrid::eq,
    );
    unique(solved).expect("Error while solving: not trivial")
}

fn step(grid: Helper<Square>, index: Coordinate<isize>) -> Helper<Square> {
    grid.0
        .get(index)
        .map(Superposition::clone)
        .map(overlaps)
        .unwrap_or_default()
        .into_iter()
        .fold(grid, |acc, c| {
            acc.0
                .adjust_at(neighbor(index, c.0), |s| restrict_tile(c.opposite(), s))
                .map(SentinelGrid)
                .unwrap_or(acc)
        })
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
    fn f() -> bool {
        true
    }
}
