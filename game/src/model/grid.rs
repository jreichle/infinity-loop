use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use quickcheck::{Arbitrary, Gen};

use super::gameboard::GameBoard;
use super::{
    accesserror::AccessError,
    tile::{Square, Tile},
};
use super::{coordinate::Coordinate, interval::Max};

/// Defines a fully filled 2D-grid with coordinate-based access
///
/// ## Layout
///
/// | ↓ row ╲ column → | 0      | 1      | 2      | ⋯ | c      |
/// |------------------|--------|--------|--------|---|--------|
/// | 0                | (0, 0) | (0, 1) | (0, 2) | ⋯ | (0, c) |
/// | 1                | (1, 0) | (1, 1) | (1, 2) | ⋯ | (1, c) |
/// | 2                | (2, 0) | (2, 1) | (2, 2) | ⋯ | (2, c) |
/// | ⋮               | ⋮      | ⋮     | ⋮     | ⋱ | ⋮     |
/// | r                | (r, 0) | (r, 1) | (r, 2) | ⋯ | (r, c) |
///
/// ## Invariants
///
/// 1. [`Grid<A>`] forms a rectangle entirely filled with elements of type [`A`]
/// 2. `∀g : Grid. g.rows * g.columns ≡ g.elements.len()`
/// 3. [`Grid`] is positioned at Coordinate (0, 0) and extends in positive directions
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Grid<A> {
    rows: usize,
    columns: usize,
    /// layout: `[(0, 0), (0, 1), (0, 2), ..., (1, 0), (1, 1), (1, 2), ...]`
    elements: Vec<A>,
}

impl<A> Grid<A> {
    /// Empty grid
    pub const EMPTY: Self = Grid {
        rows: 0,
        columns: 0,
        elements: vec![],
    };

    /// Constructs a new grid
    ///
    /// Requirement: `dimensions.product() == elements.len()`
    pub fn new(dimensions: Coordinate<usize>, elements: Vec<A>) -> Self {
        // ensure invariant #2
        let rows = dimensions.row;
        let columns = dimensions.column;
        let len = elements.len();

        assert!(
            rows * columns == len,
            "Grid::new: rows = {rows} * columns = {columns} must match elements.len = {len}",
        );
        Self {
            rows,
            columns,
            elements,
        }
    }

    /// Creates grid based on given initialization function
    pub fn init<F: Fn(Coordinate<isize>) -> A>(dimensions: Coordinate<usize>, init: F) -> Self {
        let mut elements = Vec::with_capacity(dimensions.product());

        for row in 0..dimensions.row as isize {
            for column in 0..dimensions.column as isize {
                elements.push(init(Coordinate { row, column }));
            }
        }

        Grid::new(dimensions, elements)
    }

    /// Returns the number of grid rows
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of grid columns
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// Returns the grid dimensions
    pub const fn dimensions(&self) -> Coordinate<usize> {
        Coordinate {
            row: self.rows,
            column: self.columns,
        }
    }

    /// Returns the number of elements the grid can hold
    pub const fn size(&self) -> usize {
        self.rows * self.columns
    }

    /// View of the elements in the grid
    pub fn as_slice(&self) -> &[A] {
        &self.elements[..]
    }

    ///
    pub fn elements(&self) -> Vec<A>
    where
        A: Clone,
    {
        self.elements.clone()
    }

    pub fn coordinates(&self) -> HashSet<Coordinate<isize>> {
        (0..self.rows as isize)
            .flat_map(|r| (0..self.columns as isize).map(move |c| Coordinate { row: r, column: c }))
            .collect()
    }

    pub fn zip<B, I: IntoIterator<Item = B>>(&self, iter: I) -> Grid<(A, B)>
    where
        A: Clone,
    {
        Grid {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.clone().into_iter().zip(iter).collect(),
        }
    }

    pub fn with_index(&self) -> Grid<(Coordinate<isize>, A)>
    where
        A: Clone,
    {
        let coordinates = (0..self.rows as isize).flat_map(|r| {
            (0..self.columns as isize).map(move |c| Coordinate { row: r, column: c })
        });
        Grid {
            rows: self.rows,
            columns: self.columns,
            elements: coordinates.zip(self.elements.clone()).collect(),
        }
    }

    /// see [Grid::elements] for memory layout
    fn get_vec_index(&self, index: Coordinate<isize>) -> usize {
        index.column as usize + self.columns * index.row as usize
    }

    pub fn get(&self, index: Coordinate<isize>) -> Option<&A> {
        self.ensure_index_in_bounds(index)
            .map(|_| &self.elements[self.get_vec_index(index)])
            .ok()
    }

    pub fn get_mut(&mut self, index: Coordinate<isize>) -> Option<&mut A> {
        let vec_index = self.get_vec_index(index);
        self.ensure_index_in_bounds(index)
            .map(|_| &mut self.elements[vec_index])
            .ok()
    }

    /// applies transformation to element at supplied index, if possible
    pub fn try_adjust_at<F: Fn(A) -> A>(&self, index: Coordinate<isize>, transformation: F) -> Self
    where
        A: Clone,
    {
        self.adjust_at(index, transformation)
            .unwrap_or_else(|_| self.clone())
    }

    pub fn adjust_at<F: Fn(A) -> A>(
        &self,
        index: Coordinate<isize>,
        transformation: F,
    ) -> Result<Self, AccessError>
    where
        A: Clone,
    {
        if self.ensure_index_in_bounds(index).is_ok() {
            let mut copy = self.clone();
            copy[index] = transformation(self[index].clone());
            Ok(copy)
        } else {
            Err(AccessError::IndexOutOfBounds)
        }
    }

    fn ensure_index_in_bounds(&self, index: Coordinate<isize>) -> Result<(), String> {
        if index.row >= 0
            && index.column >= 0
            && index.row < self.rows() as isize
            && index.column < self.columns() as isize
        {
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
    pub fn filled_with(dimensions: Coordinate<usize>, element: A) -> Self {
        Grid::new(dimensions, vec![element; dimensions.product()])
    }

    /// constructs Grids from array of arrays
    ///
    /// for hardcoding Grids in source code
    pub fn from_array<const R: usize, const C: usize>(elements: [[A; C]; R]) -> Self {
        Grid::new(
            Coordinate::new(R, C),
            elements.map(Vec::from).to_vec().concat(),
        )
    }

    // vec cannot be safely mapped over in-place, therefore map for Grid creates a new instance
    pub fn map<B, F: Fn(A) -> B>(&self, transform: F) -> Grid<B> {
        Grid::new(
            self.dimensions(),
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

impl<A: Clone> Grid<Option<A>> {
    pub fn sequence(&self) -> Option<Grid<A>> {
        Some(Grid {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.clone().into_iter().collect::<Option<_>>()?,
        })
    }
}

impl<A: Clone, E: Clone> Grid<Result<A, E>> {
    pub fn sequence(&self) -> Result<Grid<A>, E> {
        Ok(Grid::<A> {
            rows: self.rows,
            columns: self.columns,
            elements: self
                .elements
                .clone()
                .into_iter()
                .collect::<Result<_, _>>()?,
        })
    }
}

impl GameBoard for Grid<Tile<Square>> {
    type Index = Coordinate<isize>;

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
        } = self.dimensions().map(|x| x as isize);

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
                .iter()
                .zip(v[1..].iter())
                .all(|(tl, tr)| tl.0.contains(Square::Right) == tr.0.contains(Square::Left))
        });
        let columns_solved = (0..columns).map(column_slice).map(to_tile).all(|v| {
            v[0..]
                .iter()
                .zip(v[1..].iter())
                .all(|(tu, td)| tu.0.contains(Square::Down) == td.0.contains(Square::Up))
        });
        rows_solved && columns_solved
    }

    fn serialize_board(&self) -> std::collections::HashMap<Self::Index, &Self::Tile> {
        self.as_slice()
            .iter()
            .zip(0..)
            .map(|(x, i)| {
                (
                    Coordinate {
                        row: i / self.rows as isize,
                        column: i % self.rows as isize,
                    },
                    x,
                )
            })
            .collect()
    }
}

// Index trait is not designed to return Option
impl<A> Index<Coordinate<isize>> for Grid<A> {
    type Output = A;

    fn index(&self, index: Coordinate<isize>) -> &Self::Output {
        self.get(index).expect("Grid::index: ")
    }
}

impl<A> IndexMut<Coordinate<isize>> for Grid<A> {
    fn index_mut(&mut self, index: Coordinate<isize>) -> &mut Self::Output {
        self.ensure_index_in_bounds(index)
            .expect("Grid::index_mut: ");
        let vec_index = self.get_vec_index(index);
        &mut self.elements[vec_index]
    }
}

impl<A> Default for Grid<A> {
    fn default() -> Self {
        Self::EMPTY
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
            if self.elements.is_empty() {
                "".into()
            } else {
                self.elements
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>()
                    .chunks_exact(self.columns)
                    .map(|s| s.join(""))
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        )
    }
}

impl<A: Arbitrary> Arbitrary for Grid<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        const SIZE: usize = 10;
        let rows = Max::<SIZE>::arbitrary(g).to_usize();
        let columns = Max::<SIZE>::arbitrary(g).to_usize();
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

    use crate::model::{coordinate::Coordinate, interval::Max};

    use super::Grid;

    // restrict size grid to avoid excessive vector allocation
    #[quickcheck]
    fn ensure_dimensions(dimensions: Coordinate<Max<100>>) -> bool {
        let dimensions = dimensions.map(Max::to_usize);
        Grid::filled_with(dimensions, 0).dimensions() == dimensions
    }
}

#[cfg(test)]
mod gameboard_tests {

    use super::{GameBoard, Grid, Square, Tile};

    #[quickcheck]
    fn empty_gameboard_is_solved() -> bool {
        Grid::EMPTY.is_solved()
    }

    // single tile gameboard is solved iff tile has no connections
    #[quickcheck]
    fn single_tile_gameboard_is_solved(tile: Tile<Square>) -> bool {
        Grid::new(1.into(), vec![tile]).is_solved() == tile.0.is_empty()
    }

    // all non-mutating methods preserve the invariants by virtue of calling the invariant-preserving constructors for initialization
}
