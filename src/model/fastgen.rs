use enumset::EnumSet;
use rand::{distributions::Standard, prelude::StdRng, Rng, SeedableRng};

use super::{
    bitset::BitSet,
    coordinate::Coordinate,
    finite::Finite,
    grid::Grid,
    solver::*,
    tile::{Square, Tile},
};

///! since the tileset is not restricted and all possible tiles are available for constructing a level, any permutation of neighbor connections can be accomodated
///!
///! idea for a simplified wave function collapse algorithm:
///!     1. surround grid containing superpositions of all tiles with empty sentinel tiles
///!     2. propagate constraints of sentinel tiles
///!     3. collapse tiles randomly in a checkerboard pattern and propagate to neighbors
///!        tiles not in the checkerboard pattern are surrounded by collapsed tiles in all directions and are therefore collapsed by propagation
///!     4. delete sentinel tiles
///!
///! advantage: instead of alternately collapsing and propagating, first all are collapsed and then all are propagated
///
///! adjustment in implementation: instead of directly propagating collapsed superpositions, the whole grid is minimized at the end

impl<A: Finite> BitSet<A> {
    fn collapse_random(self, random: usize) -> Self {
        usize::checked_rem(random, self.len() as usize)
            .and_then(|i| self.into_iter().nth(i).map(BitSet::singleton))
            .unwrap_or(self)
    }
}

pub fn generate(dimension: Coordinate<usize>, seed: u64) -> Grid<Tile<Square>> {
    let sentinel = Grid::init(dimension.row, dimension.column, |_| BitSet::FULL)
        .with_sentinels(BitSet::singleton(Tile(EnumSet::empty())));
    let grid = SentinelGrid(
        minimize(sentinel)
            .0
            .with_index()
            .zip(StdRng::seed_from_u64(seed).sample_iter(Standard))
            .map(|((c, e), r)| {
                if c.sum() % 2 == 0 {
                    e.collapse_random(r)
                } else {
                    e
                }
            }),
    );
    if_unique(&minimize(grid)).unwrap()
}

/*
fn delete_connection(grid: Grid<Tile<Square>>) ->


fn generate2(dimension: Coordinate<usize>, seed: u64) -> Grid<Tile<Square>> {
    let grid = Grid::init(dimension.row, dimension.column, |c| {
        let mut set = EnumSet::all();
        let Coordinate { row: row, column: column } = c;
        if row == 0 {
            set.remove(Square::Left)
        }
        if row == dimension.row - 1 {
            set.remove(Square::Right)
        }
        if column == 0 {
            set.remove(Square::Up)
        }
        if column == dimension.column - 1 {
            set.remove(Square::Down)
        }
        Tile(set)
    });

    // delete random number of connections between tiles


    todo!()
}



struct Zero;
struct Succ<N>(N);

trait Nat {}

impl Nat for Zero {}
impl<N: Nat> Nat for Succ<N> {}

trait NatToUsize: Nat {
    const TO_USIZE: usize;
}

impl NatToUsize for Zero {
    const TO_USIZE: usize = 0;
}

impl<N: NatToUsize> NatToUsize for Succ<N> {
    const TO_USIZE: usize = 1 + N::TO_USIZE;
}


trait UsizeToNat {

    const v: Nat;

}

impl<const N: usize> UsizeToNat for usize {

}

const fn f<const N: usize>() -> [(); N] { todo!() }

trait Size {

    type Size: Nat;
}

impl Size for [(); 0] {
    type Size = Zero;
}

impl<const N: usize> Size for [(); N] {
    type Size = Succ<<[(); N-1] as Size>::Size>;
}

// Idris
// data Fin : Nat -> Type where
//    FZ : Fin (S k)
//    FS : Fin k -> Fin (S k)

trait Fin<const N: usize> {}

impl Fin<1> for Zero {}

impl<const N: usize, K: NatToUsize> Fin<N> for Succ<K> {}

// impl<const N: usize, K: Nat> Fin<N> for Succ<K> {}

enum Assert<const C: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

*/
