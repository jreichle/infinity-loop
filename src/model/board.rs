use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use quickcheck::{Arbitrary, Gen};

use super::coordinate::Coordinate;

/// naive implementation, API is bound to change
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
        Board {
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

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }
}

impl<A> Index<Coordinate<usize>> for Board<A> {
    type Output = A;

    fn index(&self, coordinate: Coordinate<usize>) -> &Self::Output {
        assert!(
            coordinate.x <= self.rows,
            "Board::index: index.x = {} should be smaller or equal to rows = {}",
            coordinate.x,
            self.rows
        );
        assert!(
            coordinate.y <= self.columns,
            "Board::index: index.y = {} should be smaller or equal to columns = {}",
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
            "Board::index: index.x = {} should be smaller or equal to rows = {}",
            coordinate.x,
            self.rows
        );
        assert!(
            coordinate.y <= self.columns,
            "Board::index: index.y = {} should be smaller or equal to columns = {}",
            coordinate.y,
            self.columns
        );
        &mut self.elements[coordinate.x + self.columns * coordinate.y]
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
        let elements = vec![A::arbitrary(g); rows * columns];
        Self {
            rows,
            columns,
            elements,
        }
    }
}
