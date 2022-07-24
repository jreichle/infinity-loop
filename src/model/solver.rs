use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;
use std::{fmt::Display, hash::Hash, ops::Not};

use enumset::{EnumSet, EnumSetType};
use quickcheck::Arbitrary;

use super::{
    bitset::BitSet,
    cardinality::Cardinality,
    coordinate::Coordinate,
    finite::Finite,
    grid::Grid,
    lattice::BoundedLattice,
    tile::{Square, Tile},
};

/// This file contains a solver algorithm
///
/// # Win Condition
///
/// A level is solved if all connections of a all tiles match their the connections of their respective neighbors. In particular this means that
///
/// 1. a tile with a connection to the right must have a neighbor to the right with a connection to the left, e.g. `[┏][━]`
/// 2. a tile without connection to the right
///     * must have a neighbor to the right without connection to the left, e.g. `[┓][┣]`
///     * is at the right edge of the level and naturally has no neighbor, e.g. `[┫]X`
///
/// These rules extend to all directions
///
/// In summary, all tile connections must be symmetric
/// This immediately implies that any solvable level must have an even number of connections
///
/// # Algorithm
///
/// ## Overview
///
/// 1. Each level consists of the set of all possible tiles in superposition
/// 2. Solving a level is achieved by extracting common features from superpositions and propagating these connection constraints to neighbors
///
/// ## Implementation
///
/// * The border rule for tiles prevents connections pointing outside the level.
/// This is functionally equivalent to having a neighbor outside the level without connection.
/// Using a layer of empty sentinel tiles around the level can be used to simplify the problem by eliminating the special border rule.
/// * square tiles under rotational symmetry form 6 equivalence classes
/// * powerset of [Tile] forms a [bounded lattice](https://en.wikipedia.org/wiki/Lattice_(order)) partially ordered by inclusion with [EnumSet::all] as maximum and [EnumSet::new] as minimum
///
///

/// enables dot notation for function calls by associating them with their respective structs

/// superposed tiles of different states, superposition with only single state is called collapsed
pub type Superposition<A> = BitSet<Tile<A>>;

/// systematic view on grid to facilitate generation and solving
pub type Sentinel<A> = SentinelGrid<Superposition<A>>;

/// Grid enclosed with a single layer of sentinel tiles
///
/// Distinct type from normal grid and serves as a helper structure to enforce the prohibition of tile connections pointing outwards
///
/// # Examples
///
/// * 🄾 = original value
/// * 🅂 = sentinel value
///
///
/// | Grid    | turns to | respective SentinelGrid |
/// |:-------:|:--------:|:-----------------------:|
/// |         |          | 🅂🅂🅂🅂🅂🅂              |
/// | 🄾🄾🄾🄾 |          | 🅂🄾🄾🄾🄾🅂              |
/// | 🄾🄾🄾🄾 | ->       | 🅂🄾🄾🄾🄾🅂              |
/// | 🄾🄾🄾🄾 |          | 🅂🄾🄾🄾🄾🅂              |
/// |         |          | 🅂🅂🅂🅂🅂🅂              |
///
/// # Invariants
///
/// * informal: tiles of the grid without neighbor in a direction gain a new sentinel value in that direction
// TODO: * formal: `forall g : Grid<A>, sg : SentinelGrid<A>, s : A. g. g.with_sentinels(s) `
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SentinelGrid<A>(pub Grid<A>);

impl<A: Clone> SentinelGrid<A> {
    /// Deletes the layer of sentinel values and returns the original grid
    pub fn extract_grid(&self) -> Grid<A> {
        Grid::init(self.0.dimensions() - Coordinate::of(2), |c| {
            self.0[c + Coordinate::of(1)].clone()
        })
    }

    /// Applies a function to all elements
    fn map<B, F: Fn(A) -> B>(&self, transform: F) -> SentinelGrid<B> {
        SentinelGrid(self.0.map(transform))
    }
}

impl<A: EnumSetType + Finite> SentinelGrid<Tile<A>> {
    /// Creates a Grid of all superimposed tiles
    ///
    /// see: [`Tile::superimpose`]
    pub fn superimpose(&self) -> Sentinel<A> {
        self.map(Tile::superimpose)
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
        Grid::arbitrary(g).with_sentinels(Tile::default())
    }
}

impl<A: Copy> Grid<A> {
    /// surrounds the whole grid with one layer of sentinel values
    ///
    /// `extract_grid` ∘ `with_sentinels` == identity
    ///
    /// function is specific to Square
    pub fn with_sentinels(&self, sentinel: A) -> SentinelGrid<A> {
        SentinelGrid(Grid::init(self.dimensions() + Coordinate::of(2), |c| {
            self.get(c - Coordinate::of(1)).copied().unwrap_or(sentinel)
        }))
    }
}

impl Square {
    /// Converts a direction to the respective delta coordinate
    fn to_coordinate(self) -> Coordinate<isize> {
        match self {
            Up => Coordinate { row: -1, column: 0 },
            Right => Coordinate { row: 0, column: 1 },
            Down => Coordinate { row: 1, column: 0 },
            Left => Coordinate { row: 0, column: -1 },
        }
    }

    // /// directions are equivalent to adding delta coordinates
    // fn functionalize(self) -> impl Fn(Coordinate<isize>) -> Coordinate<isize> {
    //     move |c| c + self.to_coordinate()
    // }
}

impl Coordinate<isize> {
    /// Converts a coordinate to the respective direction, if it is a delta coordinate
    fn to_square(self) -> Option<Square> {
        let row = self.row;
        let column = self.column;
        match (row, column) {
            (-1, 0) => Some(Up),
            (0, 1) => Some(Right),
            (1, 0) => Some(Down),
            (0, -1) => Some(Left),
            _ => None,
        }
    }

    /// Returns the position of the neighboring tile in the given direction
    ///
    /// primitive operation linking [Coordinate<usize>] and [Square]
    /// directions are fundamentally defunctionalized representations of adding delta coordinates
    fn get_neighbor_index(self, direction: Square) -> Coordinate<isize> {
        self + direction.to_coordinate()
    }

    /// Returns the position of all neighboring tiles in arbitrary order
    fn all_neighbor_indices(self) -> Vec<Coordinate<isize>> {
        BitSet::FULL
            .iter()
            .map(|dir| self.get_neighbor_index(dir))
            .collect()
    }
}

impl<A: EnumSetType + Finite> Tile<A> {
    /// Superimposes all tiles of the same equivalence class under rotational symmetry
    ///
    /// examples:
    /// * `[┗]` -> `{ [┗], [┏], [┓], [┛] }`
    /// * `[╻]` -> `{ [╹], [╺], [╻], [╸] }`
    /// * `[╋]` -> `{ [╋] }`
    fn superimpose(self) -> Superposition<A> {
        // insert successively rotated tiles until encountering repeated initial tile
        iter_fix(
            (BitSet::<Tile<A>>::EMPTY, self),
            |(s, t)| (s.inserted(*t), t.rotated_clockwise(1)),
            |x, y| x.0 == y.0,
        )
        .0
    }
}

/// unwraps if all superpositions are collapsed (= only contain single state)
pub fn extract_if_collapsed<A: Finite + Copy>(grid: &SentinelGrid<BitSet<A>>) -> Option<Grid<A>> {
    grid.extract_grid()
        .map(BitSet::unwrap_if_singleton)
        .sequence()
}

/// Ensures there is no empty superposition
///
/// An empty superposition immediately implies that the given level is without solution
///
/// The signature of function is chosen according to the rule "parse, don't validate"
/// While no parsing takes place, the caller cannot ignore the wrapped return value
fn check_no_empty_superposition<A>(
    grid: SentinelGrid<BitSet<A>>,
) -> Option<SentinelGrid<BitSet<A>>> {
    grid.0
        .as_slice()
        .iter()
        .all(|s| !s.is_empty())
        .then_some(grid)
}

impl<A: EnumSetType + Finite> Superposition<A> {
    /// restricts superposition to only include tiles with specified connection and direction
    pub fn restrict_tile(self, connection: Connection<A>) -> Self {
        let iter = self.into_iter();
        match connection {
            Connection(ref d, Status::Absent) => iter.filter(|t| !t.0.contains(*d)).collect(),
            Connection(ref d, Status::Present) => iter.filter(|t| t.0.contains(*d)).collect(),
        }
    }

    /// restricts superposition to only include tiles with the specified connection and direction
    pub fn restrict_tile2(self, connection: Connection<A>) -> Self {
        // improve by precomputing BitSets without tiles of certain connections and take intersection to achieve filtering
        self.intersection(connection.to_filter())
    }
}

impl<A: EnumSetType + Finite> Connection<A> {
    fn to_filter(&self) -> BitSet<Tile<A>> {
        BitSet::FULL
            .into_iter()
            .filter(|t: &Tile<A>| match self.1 {
                Status::Absent => !t.0.contains(self.0),
                Status::Present => t.0.contains(self.0),
            })
            .collect()
    }
}

/// memoizing combinator for unary functions
fn memoize<A: Finite, B: Copy, F: Fn(A) -> B>(f: F) -> impl FnMut(A) -> B {
    let mut cache = vec![None; A::CARDINALITY as usize];
    move |x| {
        let index = x.enum_to_index() as usize;
        match cache[index] {
            Some(x) => x,
            None => {
                let value = f(x);
                cache[index] = Some(value);
                value
            }
        }
    }
}

/// Indicates if a connection is absent oe present
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Status {
    Absent,
    Present,
}

impl Cardinality for Status {
    const CARDINALITY: u64 = 2;
}

impl Finite for Status {
    fn index_to_enum(value: u64) -> Self {
        match value % Self::CARDINALITY {
            0 => Self::Absent,
            _ => Self::Present,
        }
    }

    fn enum_to_index(&self) -> u64 {
        match self {
            Self::Absent => 0,
            Self::Present => 1,
        }
    }
}

impl Arbitrary for Status {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&[Status::Absent, Status::Present]).unwrap()
    }
}

/// Defines a connection pointing to the neighoring tile
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Connection<A>(A, Status);

impl<A: Cardinality> Cardinality for Connection<A> {
    const CARDINALITY: u64 = A::CARDINALITY * Status::CARDINALITY;
}

impl<A: Finite> Finite for Connection<A> {
    fn index_to_enum(value: u64) -> Self {
        let value = value % Self::CARDINALITY;
        Self(
            A::index_to_enum(value % A::CARDINALITY),
            Status::index_to_enum(value / A::CARDINALITY),
        )
    }

    fn enum_to_index(&self) -> u64 {
        self.0.enum_to_index() + A::CARDINALITY * self.1.enum_to_index()
    }
}

impl<A: Arbitrary> Arbitrary for Connection<A> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Connection(A::arbitrary(g), Status::arbitrary(g))
    }
}

impl Not for Connection<Square> {
    type Output = Self;

    /// Returns a connection pointing in the opposite direction
    fn not(self) -> Self::Output {
        let Connection(d, s) = self;
        Connection(d.opposite(), s)
    }
}

/// Extracts the common connections over all states from the superposition
///
/// eg. if there is a common [Up] connection between all states, then the result includes [Connection::Present(Up)]
pub fn extract_common_connections<A: EnumSetType + Finite>(
    superposition: &Superposition<A>,
) -> Vec<Connection<A>> {
    let superposition = superposition.clone();
    // an empty superposition leads to all overlaps simultaneously but since
    // an empty superposition cannot be further reduced and already signifies
    // no solution, the result does not matter
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

/// Minimizes the superpositions of the grid as far as possible through applying the logical deductions
pub fn minimize(grid: Sentinel<Square>) -> Sentinel<Square> {
    iter_fix(
        grid,
        |g| g.0.coordinates().into_iter().fold(g.clone(), step),
        SentinelGrid::eq,
    )
}

/// Chooses a tile through the supplied heuristic and for each state
/// in the superposition of that tile create a new grid with that tile in one of the states
fn branch<A, F>(grid: &Sentinel<A>, heuristic: F) -> Vec<Sentinel<A>>
where
    A: EnumSetType + Finite,
    F: Fn(&Sentinel<A>) -> Coordinate<isize>,
{
    let coordinate = heuristic(grid);
    grid.0
        .get(coordinate)
        .copied()
        .unwrap_or_default()
        .iter()
        .map(|t| SentinelGrid(grid.0.try_adjust_at(coordinate, |_| BitSet::singleton(t))))
        .collect()
}

/// Propagates all constraints from the chosen tile to all neighboring ones
pub fn step(grid: Sentinel<Square>, index: Coordinate<isize>) -> Sentinel<Square> {
    grid.0
        .get(index)
        .map(extract_common_connections)
        .unwrap_or_default()
        .into_iter()
        .fold(grid, |acc, c| {
            SentinelGrid(
                acc.0
                    .try_adjust_at(index.get_neighbor_index(c.0), |s| s.restrict_tile(!c)),
            )
        })
}

/// Returns all puzzle solutions
// hide concrete iterator implementation
pub fn solve(grid: &Grid<Tile<Square>>) -> impl Iterator<Item = Grid<Tile<Square>>> {
    SolutionIterator(vec![grid
        .with_sentinels(Tile(EnumSet::empty()))
        .superimpose()])
}

/// lazy generation of solutions to unify API for quarying single and multiple solutions
///
/// stores a stack of solution candidates, which are successively refined
struct SolutionIterator<A>(Vec<A>);

impl Iterator for SolutionIterator<Sentinel<Square>> {
    type Item = Grid<Tile<Square>>;

    /// Algorithm uses backtracking with explicit stack
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
            match extract_if_collapsed(&minimized) {
                None => {
                    // distinguish between no and several solutions
                    check_no_empty_superposition(minimized)
                        .into_iter()
                        .for_each(|g| {
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

/// witness for the ablility of [BitSet] to store at least Tile<Square>::CARDINALITY = 16 elements
const _: Superposition<Square> = BitSet::FULL;

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::tile::{Square, Tile};

    #[quickcheck]
    fn tile_configurations_have_same_number_of_connections(tile: Tile<Square>) -> bool {
        let connections = tile.0.len();
        tile.superimpose()
            .into_iter()
            .all(|t| t.0.len() == connections)
    }

    #[quickcheck]
    fn restrict_tile_sanity_check() -> bool {
        let superposition = BitSet::from_iter(Tile(Up | Right).superimpose());
        let connection = Connection(Right, Status::Present);
        superposition.restrict_tile(connection)
            == BitSet::from_iter([Tile(Up | Right), Tile(Right | Down)])
    }

    #[quickcheck]
    fn neighborhood_is_symmetric(index: Coordinate<i8>, direction: Square) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(|x| x as isize);
        let neighbor_index = index.get_neighbor_index(direction);
        index == neighbor_index.get_neighbor_index(direction.opposite())
    }

    /// successively visiting neighbors in each
    #[quickcheck]
    fn neighborhood_is_euclidian(index: Coordinate<i8>) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(|x| x as isize);
        BitSet::FULL
            .into_iter()
            .fold(index, Coordinate::get_neighbor_index)
            == index
    }

    #[quickcheck]
    fn with_sentinels_and_then_extract_grid_is_id(
        grid: Grid<Tile<Square>>,
        sentinel: Tile<Square>,
    ) -> bool {
        grid == grid.with_sentinels(sentinel).extract_grid()
    }
}
