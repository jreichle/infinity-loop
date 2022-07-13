use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;
use std::{fmt::Display, hash::Hash};

use enumset::EnumSetType;
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

/// simplifying function calls by associating them with their respective structs

pub type Superposition<A> = BitSet<Tile<A>>;

/// systematic view on grid to facilitate construction and solving
pub type Sentinel<A> = SentinelGrid<Superposition<A>>;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SentinelGrid<A>(pub Grid<A>);

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

impl<A: EnumSetType + Finite> SentinelGrid<Tile<A>> {
    /// grid of superimposed tiles in different configurations
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
        SentinelGrid(Grid::init(self.rows() + 2, self.columns() + 2, |c| {
            self.get(c - Coordinate::of(1)).copied().unwrap_or(sentinel)
        }))
    }
}

impl Square {
    fn to_coordinate(self) -> Coordinate<isize> {
        match self {
            Up => Coordinate { row: -1, column: 0 },
            Right => Coordinate { row: 0, column: 1 },
            Down => Coordinate { row: 1, column: 0 },
            Left => Coordinate { row: 0, column: -1 },
        }
    }
}

impl Coordinate<isize> {
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

    /// primitive operation linking [Coordinate<usize>] and [Square]
    /// coordinate system exists outside of the grid
    fn neighbor(self, direction: Square) -> Coordinate<isize> {
        self + direction.to_coordinate()
    }

    fn all_neighbors(self) -> Vec<Coordinate<isize>> {
        BitSet::FULL.iter().map(|dir| self.neighbor(dir)).collect()
    }
}

impl<A: EnumSetType + Finite> Tile<A> {
    /// expands tile to a the superposition of all element of the tiles equivalence class under rotational symmetry
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
pub fn if_unique<A: Finite + Copy>(grid: &SentinelGrid<BitSet<A>>) -> Option<Grid<A>> {
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

impl<A: EnumSetType + Finite> Superposition<A> {
    /// restricts superposition to only include tiles with specified connection and direction
    pub fn restrict_tile(self, connection: Connection<A>) -> Self {
        let iter = self.into_iter();
        match connection {
            Connection(ref d, Status::Absent) => iter.filter(|t| !t.0.contains(*d)).collect(),
            Connection(ref d, Status::Present) => iter.filter(|t| t.0.contains(*d)).collect(),
        }
    }

    /// restricts superposition to only include tiles with specified connection and direction
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

/// memoizing combinator for 1-ary functions
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
pub fn extract_overlaps<A: EnumSetType + Finite>(
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

pub fn minimize(grid: Sentinel<Square>) -> Sentinel<Square> {
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

pub fn step(grid: Sentinel<Square>, index: Coordinate<isize>) -> Sentinel<Square> {
    grid.0
        .get(index)
        .map(Superposition::clone)
        .map(extract_overlaps)
        .unwrap_or_default()
        .into_iter()
        .fold(grid, |acc, c| {
            SentinelGrid(
                acc.0
                    .try_adjust_at(index.neighbor(c.0), |s| s.restrict_tile(c.opposite())),
            )
        })
}

// hide concrete iterator implementation
pub fn solve(grid: &Grid<Tile<Square>>) -> impl Iterator<Item = Grid<Tile<Square>>> {
    SolutionIterator(vec![grid.with_sentinels(Tile::default()).superimpose()])
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
        let neighbor_index = index.neighbor(direction);
        index == neighbor_index.neighbor(direction.opposite())
    }

    /// successively visiting neighbors in each
    #[quickcheck]
    fn neighborhood_is_euclidian(index: Coordinate<i8>) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(|x| x as isize);
        BitSet::FULL.into_iter().fold(index, Coordinate::neighbor) == index
    }

    #[quickcheck]
    fn with_sentinels_and_then_extract_grid_is_id(
        grid: Grid<Tile<Square>>,
        sentinel: Tile<Square>,
    ) -> bool {
        grid == grid.with_sentinels(sentinel).extract_grid()
    }
}
