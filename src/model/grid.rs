use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use quickcheck::{Arbitrary, Gen};

use super::coordinate::Coordinate;
use super::gameboard::GameBoard;
use super::{
    accesserror::AccessError,
    tile::{Square, Tile},
};

/// gameboard as 2D-grid implemented with a flattened [Vec]
///
/// defines the geometry of the puzzle
///
/// * invariant through game design: gameboard forms a recangle completely filled with [Tile]s
/// * invariant: ∀g: Grid. g.rows * g.columns == g.elements.len()
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Grid<A> {
    pub rows: usize,
    pub columns: usize,
    elements: Vec<A>,
}

impl<A> Grid<A> {
    /// primary constructor
    pub fn new(rows: usize, columns: usize, elements: Vec<A>) -> Self {
        // ensure all invariants hold
        assert!(
            rows * columns == elements.len(),
            "Grid::new: rows = {rows} * columns = {columns} must match length of elements = {}",
            elements.len()
        );
        Self {
            rows,
            columns,
            elements,
        }
    }

    pub fn init<F: Fn(usize, usize) -> A>(rows: usize, columns: usize, init: F) -> Self {
        let mut elements = Vec::with_capacity(rows * columns);

        for column in 0..columns {
            for row in 0..rows {
                elements.push(init(row, column));
            }
        }

        Grid::new(rows, columns, elements)
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }

    pub const fn dimensions(&self) -> Coordinate<usize> {
        Coordinate {
            row: self.rows,
            column: self.columns,
        }
    }

    pub const fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn elements(&self) -> &[A] {
        &self.elements[..]
    }

    pub fn elements2(&self) -> Vec<A>
    where
        A: Clone,
    {
        self.elements.clone()
    }

    // pub fn get_mut(&mut self, index: Coordinate<usize>) -> Option<&mut A>
    // where
    //     A: Copy,
    // {
    //     match index {
    //         Coordinate { row, column } if row <= self.rows() && column <= self.columns() => {
    //             Some(&mut self.elements[row * self.columns + column])
    //         }
    //         _ => None,
    //     }
    // }

    // /// applies transformation to element at supplied index, if possible
    // pub fn adjust_at<F: FnOnce(A) -> A>(&self, index: Coordinate<usize>, transformation: F) -> Self
    // where
    //     A: Copy + Clone,
    // {
    //     let mut copy = self.clone();
    //     if self.ensure_index_in_bounds(index).is_ok() {
    //         copy[index] = transformation(self[index]);
    //     }
    //     copy
    // }

    pub fn adjust_at<F: FnOnce(A) -> A>(
        &self,
        index: Coordinate<usize>,
        transformation: F,
    ) -> Result<Self, AccessError>
    where
        A: Copy + Clone,
    {
        if self.ensure_index_in_bounds(index).is_ok() {
            let mut copy = self.clone();
            copy[index] = transformation(self[index]);
            Ok(copy)
        } else {
            Err(AccessError::IndexOutOfBounds)
        }
    }

    fn ensure_index_in_bounds(&self, index: Coordinate<usize>) -> Result<(), String> {
        if index.row <= self.rows() && index.column <= self.columns() {
            Ok(())
        } else {
            Err(format!(
                "grid dimensions are {}, but trying to access {}",
                self.dimensions(),
                index
            ))
        }
    }
}

impl<A: Clone> Grid<A> {
    pub fn filled_with(rows: usize, columns: usize, element: A) -> Self {
        Grid::new(rows, columns, vec![element; rows * columns])
    }

    /// constructs Grids from array of arrays
    ///
    /// for hardcoding Grids in source code
    pub fn from_array<const R: usize, const C: usize>(elements: [[A; R]; C]) -> Self {
        Grid::new(
            elements.len(),
            elements.get(0).map(|x| x.len()).unwrap_or(0),
            elements.map(Vec::from).to_vec().concat(),
        )
    }

    // vec cannot be safely mapped over in-place, therefore map for Grid creates a new instance
    pub fn map<B, F: Fn(A) -> B>(&self, transform: F) -> Grid<B> {
        Grid::new(
            self.rows,
            self.columns,
            self.elements.clone().into_iter().map(transform).collect(),
        )
    }

    fn map_mut<F: FnMut(&mut A) -> A>(mut self, mut transform: F) -> Self {
        for v in &mut self.elements {
            *v = transform(v);
        }
        self
    }
}

impl GameBoard for Grid<Tile<Square>> {
    type Index = Coordinate<usize>;

    type Tile = Tile<Square>;

    fn rotate_clockwise(&self, index: Self::Index) -> Result<Self, AccessError> {
        self.adjust_at(index, |x| x.rotated_clockwise(1))
    }

    fn rotate_counterclockwise(&self, index: Self::Index) -> Result<Self, AccessError> {
        self.adjust_at(index, |x| x.rotated_counterclockwise(1))
    }

    fn is_solved(&self) -> bool {
        // currently implemented as pure function on gameboard without caching
        // in case of performance issues use caching of already solved grid regions
        //
        // algorithm
        //
        // split up grid in all complete horizontal and vertical sections by coordinates
        // transform coordinates into respective tiles and enclose each section with empty sentinel tiles
        // to ensure absence of connections pointing outside the grid
        // ensure all neighboring tiles in sections have matching connections, either both or neither connection pointing to each other
        //
        // Tile is newtype around integer, owned vs borrowed irrelevant

        // better algorithm
        // copy into new grid with borde of sentinel values
        // for each (coord, v) except last column check connection to right neighbor
        // for each (coord, v) except last row check connection to down neighbor

        let Coordinate {
            row: rows,
            column: columns,
        } = self.dimensions();

        let enclose_sentinels = |mut v: Vec<Self::Tile>| {
            v.insert(0, Self::Tile::default());
            v.push(Self::Tile::default());
            v
        };
        let row_slice = |r| {
            (0..columns)
                .map(|c| Coordinate { row: r, column: c })
                .collect::<Vec<_>>()
        };
        let column_slice = |c| {
            (0..rows)
                .map(|r| Coordinate { row: r, column: c })
                .collect::<Vec<_>>()
        };

        let to_tile =
            |v: Vec<Coordinate<_>>| enclose_sentinels(v.into_iter().map(|c| self[c]).collect());

        let rows_solved = (0..rows).map(row_slice).map(to_tile).all(|v| {
            v[0..]
                .into_iter()
                .zip(v[1..].into_iter())
                .all(|(tl, tr)| tl.0.contains(Square::Right) == tr.0.contains(Square::Left))
        });
        let columns_solved = (0..columns).map(column_slice).map(to_tile).all(|v| {
            v[0..]
                .into_iter()
                .zip(v[1..].into_iter())
                .all(|(tu, td)| tu.0.contains(Square::Down) == td.0.contains(Square::Up))
        });
        rows_solved && columns_solved
    }

    fn serialize_board(&self) -> std::collections::HashMap<Self::Index, &Self::Tile> {
        self.elements()
            .into_iter()
            .zip(0..)
            .map(|(x, i)| {
                (
                    Coordinate {
                        row: i / self.rows,
                        column: i % self.rows,
                    },
                    x,
                )
            })
            .collect()
    }
}

// Index trait is not designed to return Option
impl<A> Index<Coordinate<usize>> for Grid<A> {
    type Output = A;

    fn index(&self, index: Coordinate<usize>) -> &Self::Output {
        self.ensure_index_in_bounds(index).expect("Grid::index: ");
        &self.elements[index.row + self.columns * index.column]
    }
}

impl<A> IndexMut<Coordinate<usize>> for Grid<A> {
    fn index_mut(&mut self, index: Coordinate<usize>) -> &mut Self::Output {
        self.ensure_index_in_bounds(index)
            .expect("Grid::index_mut: ");
        &mut self.elements[index.row + self.columns * index.column]
    }
}

impl<A> IntoIterator for Grid<A> {
    type Item = A;

    type IntoIter = IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<A: Display> Display for Grid<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|x| format!("{x}"))
                .collect::<Vec<String>>()
                .chunks_exact(self.columns)
                .map(|s| s.join(""))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl<A: Arbitrary> Arbitrary for Grid<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        let rows = usize::arbitrary(g);
        let columns = usize::arbitrary(g);
        let elements = (0..rows * columns).map(|_| A::arbitrary(g)).collect();
        Self {
            rows,
            columns,
            elements,
        }
    }
}

#[cfg(test)]
mod grid_tests {

    use super::Grid;

    // restrict size of rows and columns to avoid excessive vector allocation
    #[quickcheck]
    fn ensure_dimensions(rows: u8, columns: u8) -> bool {
        let rows = rows as usize;
        let columns = columns as usize;
        let board = Grid::filled_with(rows, columns, 0);
        board.rows() == rows && board.columns() == columns
    }
}

#[cfg(test)]
mod gameboard_tests {

    use super::{GameBoard, Grid, Square, Tile};

    #[quickcheck]
    fn empty_gameboard_is_solved() -> bool {
        Grid::new(0, 0, vec![]).is_solved()
    }

    // single tile gameboard is solved iff tile has no connections
    #[quickcheck]
    fn single_tile_gameboard_is_solved(tile: Tile<Square>) -> bool {
        let is_solved = Grid::new(1, 1, vec![tile]).is_solved();
        if tile.0.is_empty() {
            is_solved
        } else {
            !is_solved
        }
    }
}
