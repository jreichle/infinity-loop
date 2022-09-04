use crate::model::tile::Square::{Down, Left, Right, Up};
use core::fmt::Debug;

use std::{
    fmt::Display,
    hash::Hash,
    ops::{Neg, Not},
};

use quickcheck::Arbitrary;

use super::{
    coordinate::Coordinate,
    enummap::EnumMap,
    enumset::EnumSet,
    finite::Finite,
    grid::Grid,
    lattice::BoundedLattice,
    tile::{Square, Tile},
    cnf
};

///! This file contains a solver algorithm
///!
///! # Win Condition
///!
///! A level is solved if all connections of a all tiles match the connections of their respective neighbors. In particular this means that
///!
///! 1. a tile with a connection to the right must have a neighbor to the right with a connection to the left, e.g. `[┏][━]`
///! 2. a tile without connection to the right
///!     * must have a neighbor to the right without connection to the left, e.g. `[┓][┣]`
///!     * is at the right edge of the level and naturally has no neighbor, e.g. `[┫]X`
///!
///! These rules extend to all directions
///!
///! In summary, all tile connections must be symmetric
///! This immediately implies that any solvable level must have an even number of connections
///!
///! # Algorithm
///!
///! ## Overview
///!
///! 1. Each level consists of the set of all possible tiles in superposition
///! 2. Solving a level is achieved by extracting common features from superpositions and propagating these connection constraints to neighbors
///!
///! ## Implementation
///!
///! * The border rule for tiles prevents connections pointing outside the level.
///! This is functionally equivalent to having a neighbor outside the level without connection.
///! Using a layer of empty sentinel tiles around the level can be used to simplify the problem by eliminating the special border rule.
///! * square tiles under rotational symmetry form 6 equivalence classes
///! 
///!

// enable dot notation for function calls by associating them with their respective structs for autocomplete and to avoid parantheses

/// set of superimposed tiles in different states, superposition with only single state is called collapsed
pub type Superposition<A> = EnumSet<Tile<A>>;

/// systematic view on grid to facilitate generation and solving
pub type Sentinel<A> = SentinelGrid<Superposition<A>>;

/// Grid enclosed with a single layer of sentinel tiles
///
/// It is a distinct type through wrapping the grid struct and serves as a helper structure to enforce the rule that tile connections must not point outwards
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
        Grid::init(self.0.dimensions() - 2, |c| self.0[c + 1].clone())
    }

    /// Applies a function to all elements of the SentinelGrid
    fn map<B, F: Fn(A) -> B>(&self, transform: F) -> SentinelGrid<B> {
        SentinelGrid(self.0.map(transform))
    }
}

impl<A: Finite + Clone> SentinelGrid<Tile<A>> {
    /// Creates a Grid of all superimposed tiles
    ///
    /// see: [`Tile::superimpose`]
    pub fn superimpose(&self) -> Sentinel<A> {
        self.map(Tile::superimpose)
    }
}

impl<A: Clone + Display + Finite + BoundedLattice + Iterator> Display for SentinelGrid<A>
where
    <A as IntoIterator>::Item: Debug + BoundedLattice + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.map(BoundedLattice::or).fmt(f)
    }
}

impl<A: Clone + Finite + 'static> Arbitrary for SentinelGrid<Tile<A>> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Grid::arbitrary(g).with_sentinels(Tile::NO_CONNECTIONS)
    }
}

impl<A: Clone + Finite + 'static> Arbitrary for SentinelGrid<Superposition<A>> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Grid::arbitrary(g).with_sentinels(Tile::NO_CONNECTIONS.into())
    }
}

impl<A: Copy> Grid<A> {
    /// surrounds the whole grid with one layer of sentinel values
    ///
    /// `extract_grid` ∘ `with_sentinels` == identity
    ///
    /// function is specific to Square
    pub fn with_sentinels(&self, sentinel: A) -> SentinelGrid<A> {
        SentinelGrid(Grid::init(self.dimensions() + 2, |c| {
            self.get(c - 1).copied().unwrap_or(sentinel)
        }))
    }
}

impl Square {
    /// Converts a direction to the respective delta coordinate
    fn to_coordinate(self) -> Coordinate<isize> {
        match self {
            Up => (-1, 0).into(),
            Right => (0, 1).into(),
            Down => (1, 0).into(),
            Left => (0, -1).into(),
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
        Some(match self.to_tuple() {
            (-1, 0) => Up,
            (0, 1) => Right,
            (1, 0) => Down,
            (0, -1) => Left,
            _ => None?,
        })
    }

    /// Returns the position of the neighboring tile in the given direction
    ///
    /// primitive operation linking [Coordinate<usize>] and [Square]
    /// directions are fundamentally defunctionalized representations of adding delta coordinates
    pub fn get_neighbor_index(self, direction: Square) -> Coordinate<isize> {
        self + direction.to_coordinate()
    }

    /// Returns the position of all neighboring tiles in arbitrary order
    pub fn all_neighbor_indices(self) -> Vec<Coordinate<isize>> {
        EnumSet::FULL
            .iter()
            .map(|dir| self.get_neighbor_index(dir))
            .collect()
    }
}

impl<A: Finite> Tile<A> {
    /// Superimposes all tiles of the same equivalence class under rotational symmetry
    ///
    /// examples:
    /// * `[┗]` -> `{ [┗], [┏], [┓], [┛] }`
    /// * `[╻]` -> `{ [╹], [╺], [╻], [╸] }`
    /// * `[╋]` -> `{ [╋] }`
    pub fn superimpose(self) -> Superposition<A> {
        // insert successively rotated tiles until encountering repeated initial tile
        iter_fix(
            (EnumSet::<Tile<A>>::EMPTY, self),
            |(s, t)| (s.inserted(*t), t.rotated_clockwise(1)),
            |x, y| x.0 == y.0,
        )
        .0
    }
}

impl<A: Finite + Copy> SentinelGrid<EnumSet<A>> {
    /// unwraps if all superpositions are collapsed (= only contain single state)
    pub fn extract_if_collapsed(&self) -> Option<Grid<A>> {
        self.extract_grid()
            .map(EnumSet::unwrap_if_singleton)
            .sequence()
    }
}

impl<A> SentinelGrid<EnumSet<A>> {
    /// Ensures there is no empty superposition
    ///
    /// An empty superposition immediately implies that the given level is without solution
    ///
    /// The signature of function is chosen according to the rule "parse, don't validate"
    /// While no parsing takes place, the caller cannot ignore the wrapped return value
    fn check_no_empty_superposition(self) -> Option<SentinelGrid<EnumSet<A>>> {
        self.0
            .as_slice()
            .iter()
            .all(|s| !s.is_empty())
            .then_some(self)
    }
}

/// memoizing combinator for unary functions
fn memoize<A: Finite, B: Copy, F: Fn(A) -> B>(f: F) -> impl FnMut(A) -> B {
    let mut cache = vec![None; A::CARDINALITY as usize];
    move |x| {
        let index = x.enum_to_index() as usize;
        match cache[index] {
            Some(value) => value,
            None => {
                let value = f(x);
                cache[index] = Some(value);
                value
            }
        }
    }
}

impl<A: Copy + Finite + Neg<Output = A> + PartialEq> Superposition<A> {
    /// Generates all propagation information from on a single superposition
    pub fn extract_common_connections(self) -> EnumMap<A, Superposition<A>> {
        let present_evidence = BoundedLattice::and(self)
            .0
            .into_iter()
            .map(|x| (x, subset_containing(-x)));
        let absent_evidence = BoundedLattice::or(self)
            .0
            .not()
            .into_iter()
            .map(|x| (x, !subset_containing(-x)));
        present_evidence.chain(absent_evidence).collect()
    }
}

fn subset_containing<A: Copy + Finite>(value: A) -> Superposition<A> {
    EnumSet::FULL
        .into_iter()
        .filter(|s: &Tile<A>| s.0.contains(value))
        .collect()
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

impl Sentinel<Square> {
    /// Minimizes the superpositions of the grid as far as possible through applying the logical deductions
    ///
    /// notably, how the function achieves this is an implementation detail
    ///
    /// # Postcondition
    ///
    /// `s.minimize()` ≡ `s.minimize().minimize()`
    pub fn minimize(self) -> Sentinel<Square> {
        iter_fix(
            self,
            |g| {
                g.0.coordinates()
                    .into_iter()
                    .fold(g.clone(), |g, c| propagate_restrictions_to_all_neighbors2(g, c, PartialEq::ne).0)
            },
            SentinelGrid::eq,
        )
    }
}

impl<A: Finite> Sentinel<A> {
    /// Chooses a tile through the supplied heuristic and for each state
    /// in the superposition of that tile create a new grid with that tile in one of the states
    fn branch<F>(&self, heuristic: F) -> Vec<Sentinel<A>>
    where
        F: Fn(&Sentinel<A>) -> Coordinate<isize>,
    {
        let coordinate = heuristic(self);
        self.0
            .get(coordinate)
            .copied()
            .unwrap_or(EnumSet::EMPTY)
            .iter()
            .map(|t| SentinelGrid(self.0.try_adjust_at(coordinate, |_| t.into())))
            .collect()
    }
}
/// Splits the superposition with the most states
fn most_superimposed_states<A: Finite>(grid: &Sentinel<A>) -> Coordinate<isize> {
    grid.0
        .coordinates()
        .into_iter()
        .max_by_key(|c| grid.0[*c].len())
        .expect("Logical error: attempted to branch, but was unable")
}

// TODO: split to isolate parts only working on Square,
/// Propagates all constraints from the chosen tile to all neighboring ones
pub fn propagate_restrictions_to_all_neighbors(
    grid: Sentinel<Square>,
    index: Coordinate<isize>,
) -> Sentinel<Square> {
    // determine common connections
    let evidence = grid
        .0
        .get(index)
        .copied()
        .map(Superposition::extract_common_connections)
        .unwrap_or_default();

    // propagate connection information to neighbors
    evidence.into_iter().fold(grid, |acc, c| {
        SentinelGrid(
            acc.0
                .try_adjust_at(index.get_neighbor_index(c.0), |s| s & c.1),
        )
    })
}
// for solving change_test is inequality, for hint it is collapse
pub fn propagate_restrictions_to_all_neighbors2<F: FnMut(&Superposition<Square>, &Superposition<Square>) -> bool>(
    grid: Sentinel<Square>,
    index: Coordinate<isize>,
    mut change_test: F, 
) -> (Sentinel<Square>, Vec<Coordinate<isize>>) {
    // determine common connections
    let evidence = grid
        .0
        .get(index)
        .copied()
        .map(Superposition::extract_common_connections)
        .unwrap_or_default();

    // propagate connection information to neighbors
    // pushing to vec as side effect is unclean, but easiest implementation 
    evidence.into_iter().fold((grid, vec![]), |(g, mut v), c| {
        let neighbor_index = index.get_neighbor_index(c.0);
        let sg = SentinelGrid(g.0.try_adjust_at(neighbor_index, |s| {
            let merged = s & c.1;
            if change_test(&s, &merged) {
                // pushing to vec as side effect is unclean, but easiest implementation 
                v.push(neighbor_index);
            }
            merged
        }));
        (sg, v)
    })
}

impl Grid<Tile<Square>> {
    /// Returns all puzzle solutions
    // hide concrete iterator implementation
    // solves puzzles up to 20x20 reasonably fast
    pub fn solve(&self) -> impl Iterator<Item = Grid<Tile<Square>>> {
        SolutionIterator(vec![self
            .with_sentinels(Tile::NO_CONNECTIONS)
            .superimpose()])
    }

    //takes a user supplied input and runs solved_to_tiles
    //if that did not generate a sufficient solution an unsolvable puzzle is generated to handle this error
    pub fn solve_with_input(&self, input: &str) -> Grid<Tile<Square>> {
        let tiles = cnf::solved_to_tiles(input).unwrap();
        if tiles.len() == self.columns() * self.rows() {
            Grid::new(Coordinate::new(self.columns(),self.rows()),tiles)
        }
        else {
            let mut unsolvable = vec![];
            for _i in 0..self.columns()*self.rows() {
                unsolvable.push(Tile::ALL_CONNECTIONS);
            }
            Grid::new(Coordinate::new(self.columns(),self.rows()), unsolvable)
        }
        
    }
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
    /// * __contradiction__: pop next element from stack and repeat procedure
    /// * __unique solution__: return in solved state
    /// * __branching__: select 1 candidate and repeat procedure, push rest on stack
    ///
    /// it is yet to be determined if a contradiction after branching can actually occur
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let minimized_grid = self.0.pop()?.minimize();

            // yield, if unique solution
            if let Some(grid) = minimized_grid.extract_if_collapsed() {
                return Some(grid);
            }

            // distinguish between no and several solutions
            if let Some(grid) = minimized_grid.check_no_empty_superposition() {
                self.0.extend(grid.branch(most_superimposed_states))
            }
        }
    }
}

/// witness for the ablility of [`EnumSet`] to store at least [`Tile<Square>::CARDINALITY`] = 16 elements
const _: Superposition<Square> = EnumSet::FULL;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::model::interval::Max;

    #[quickcheck]
    fn tile_configurations_have_same_number_of_connections(tile: Tile<Square>) -> bool {
        let connections = tile.0.len();
        tile.superimpose()
            .into_iter()
            .all(|t| t.0.len() == connections)
    }

    #[quickcheck]
    fn neighborhood_is_symmetric(index: Coordinate<Max<100>>, direction: Square) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(Max::to_isize);
        let neighbor_index = index.get_neighbor_index(direction);
        index == neighbor_index.get_neighbor_index(-direction)
    }

    /// successively visiting neighbors in each direction
    #[quickcheck]
    fn neighborhood_is_euclidian(index: Coordinate<Max<100>>) -> bool {
        // restrict coordinates to a range resembling actual values used in grid and avoid integer over- / underflows
        let index = index.map(Max::to_isize);
        EnumSet::FULL
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
