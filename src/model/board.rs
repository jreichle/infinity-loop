use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    vec::IntoIter,
};

use quickcheck::{Arbitrary, Gen};

use super::coordinate::Coordinate;

/// gameboard as 2D-grid implemented with a flattened [Vec]
///
/// invariant through game design: board forms a recangle completely filled with [Tile]s
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Board<A> {
    rows: usize,
    columns: usize,
    elements: Vec<A>,
}

impl<A> Board<A> {
    pub fn new(rows: usize, columns: usize, elements: Vec<A>) -> Self {
        assert!(
            rows * columns == elements.len(),
            "Board::new: rows = {rows} * columns = {columns} must match length of elements = {}",
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
        for y in 0..columns {
            for x in 0..rows {
                elements.push(init(x, y));
            }
        }
        Self {
            rows,
            columns,
            elements,
        }
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }

    pub const fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn elements(&self) -> &[A] {
        &self.elements[..]
    }
}

impl<A: Clone> Board<A> {
    pub fn filled_with(rows: usize, columns: usize, element: A) -> Self {
        Self {
            rows,
            columns,
            elements: vec![element; rows * columns],
        }
    }
}

impl<A> Index<Coordinate<usize>> for Board<A> {
    type Output = A;

    fn index(&self, coordinate: Coordinate<usize>) -> &Self::Output {
        assert!(
            coordinate.x <= self.rows,
            "Board::index: coordinate.x = {} should be smaller or equal to rows = {}",
            coordinate.x,
            self.rows
        );
        assert!(
            coordinate.y <= self.columns,
            "Board::index: coordinate.y = {} should be smaller or equal to columns = {}",
            coordinate.y,
            self.columns
        );
        &self.elements[coordinate.x + self.columns * coordinate.y]
    }
}

impl<A> IndexMut<Coordinate<usize>> for Board<A> {
    fn index_mut(&mut self, coordinate: Coordinate<usize>) -> &mut Self::Output {
        assert!(
            coordinate.x <= self.rows,
            "Board::index_mut: coordinate.x = {} should be smaller or equal to rows = {}",
            coordinate.x,
            self.rows
        );
        assert!(
            coordinate.y <= self.columns,
            "Board::index_mut: coordinate.y = {} should be smaller or equal to columns = {}",
            coordinate.y,
            self.columns
        );
        &mut self.elements[coordinate.x + self.columns * coordinate.y]
    }
}

impl<A> IntoIterator for Board<A> {
    type Item = A;

    type IntoIter = IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<A: Display> Display for Board<A> {
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

impl<A: Arbitrary> Arbitrary for Board<A> {
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
mod tests {

    use crate::Board;

    // restrict size of rows and columns to avoid excessive vector allocation
    #[quickcheck]
    fn ensure_dimensions(rows: u8, columns: u8) -> bool {
        let rows = rows as usize;
        let columns = columns as usize;
        let board = Board::filled_with(rows, columns, 0);
        board.rows() == rows && board.columns() == columns
    }
}
