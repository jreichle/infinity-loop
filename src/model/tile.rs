use std::{
    fmt::Display,
    ops::{BitOr, Not},
};

use quickcheck::{Arbitrary, Gen};
use Square::{Down, Left, Right, Up};

use super::{enumset::EnumSet, cardinality::Cardinality, finite::Finite};

/// Represents a direction for a tile connection
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    /// Coordinate(-1, 0)
    Up,
    /// Coordinate(0, 1)
    Right,
    /// Coordinate(1, 0)
    Down,
    /// Coordinate(0, -1)
    Left,
}

impl Square {
    /// Returns the opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

impl BitOr for Square {
    type Output = EnumSet<Square>;

    fn bitor(self, rhs: Self) -> Self::Output {
        EnumSet::singleton(self).inserted(rhs)
    }
}

impl<A: Finite> BitOr<A> for EnumSet<A> {
    type Output = EnumSet<A>;

    fn bitor(self, rhs: A) -> Self::Output {
        self.inserted(rhs)
    }
}

impl Not for Square {
    type Output = Square;

    fn not(self) -> Self::Output {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => "Up",
                Right => "Right",
                Down => "Down",
                Left => "Left",
            }
        )
    }
}

impl Arbitrary for Square {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&[Up, Right, Down, Left]).unwrap()
    }
}

impl Cardinality for Square {
    const CARDINALITY: u64 = 4;
}

impl Finite for Square {
    fn index_to_enum(value: u64) -> Self {
        match value % 4 {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            _ => Self::Left,
        }
    }

    fn enum_to_index(&self) -> u64 {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }
}

/// Tilable 2D shape with individual binary connection indicators towards neighboring tiles
///
/// The possible directions for connections correspond to the number of enum values in the EnumSetType
///
/// it is guaranteed to implement the [Copy] trait
///
/// Basic operations are checking present connections and rotating tiles
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile<A>(pub EnumSet<A>);

impl<A> Clone for Tile<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A> Copy for Tile<A> {}

impl<A> Default for Tile<A> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<A> Tile<A> {
    pub const EMPTY: Tile<A> = Self(EnumSet::EMPTY);
}

impl<A: Cardinality> Tile<A> {
    pub const FULL: Tile<A> = Self(EnumSet::FULL);
}

impl<A: Finite> Tile<A> {
    /// Rotates the tile clockwise by 360° / [`A::CARDINALITY`]
    ///
    /// Performing [`A::CARDINALITY`] rotations returns the original tile
    pub fn rotated_clockwise(&self, repetitions: u64) -> Self {
        let bit_rep = self.enum_to_index();
        let repetitions = repetitions % A::CARDINALITY;
        let rotated_bit_rep =
            (bit_rep << repetitions) | (bit_rep >> (A::CARDINALITY - repetitions));
        Self(EnumSet::index_to_enum(rotated_bit_rep))
    }

    /// Rotates the tile counterclockwise by 360° / [`A::CARDINALITY`]
    ///
    /// Performing [`A::CARDINALITY`] rotations returns the original tile
    pub fn rotated_counterclockwise(&self, repetitions: u64) -> Self {
        self.rotated_clockwise(A::CARDINALITY - repetitions % A::CARDINALITY)
    }
}

impl<A: Cardinality> Not for Tile<A> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<A: Cardinality> Cardinality for Tile<A> {
    const CARDINALITY: u64 = EnumSet::<A>::CARDINALITY;
}

impl<A: Finite> Finite for Tile<A> {
    fn index_to_enum(value: u64) -> Self {
        Self(EnumSet::index_to_enum(value))
    }

    fn enum_to_index(&self) -> u64 {
        self.0.enum_to_index()
    }
}

impl<A: 'static + Clone + Finite> Arbitrary for Tile<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(EnumSet::index_to_enum(u64::arbitrary(g)))
    }
}

impl Display for Tile<Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index: usize = self.0.iter().map(|s| 1 << s.enum_to_index()).sum();
        write!(f, "{}", Self::UNICODE_TILES[index])
    }
}

impl Tile<Square> {
    /// visualization for each tile state as UNICODE character
    const UNICODE_TILES: [char; 16] = [
        ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
    ];

    #[deprecated(
        note = "please use Tile::to_string to display the tile instead or use a Tile object"
    )]
    pub fn get_value(&self) -> char {
        let index: usize = self.0.iter().map(|s| 1 << s.enum_to_index()).sum();
        Self::UNICODE_TILES[index]
    }
}

#[cfg(test)]
mod tests {

    use crate::model::cardinality::Cardinality;

    use super::{Square, Tile};

    #[quickcheck]
    fn rotate_clockwise_cardinality_times_is_identity(tile: Tile<Square>) -> bool {
        tile == tile.rotated_clockwise(Square::CARDINALITY)
    }

    #[quickcheck]
    fn repeated_clockwise_rotation_is_clockwise_rotation_with_repetitions(
        tile: Tile<Square>,
        repetitions: u8,
    ) -> bool {
        let repeated = (0..repetitions).fold(tile, |acc, _| acc.rotated_clockwise(1));

        repeated == tile.rotated_clockwise(repetitions as u64)
    }

    #[quickcheck]
    fn rotate_clockwise_preserves_number_of_connections(
        tile: Tile<Square>,
        rotations: u64,
    ) -> bool {
        tile.0.len() == tile.rotated_clockwise(rotations).0.len()
    }

    #[quickcheck]
    fn rotate_counterclockwise_cardinality_times_is_identity(tile: Tile<Square>) -> bool {
        tile == tile.rotated_counterclockwise(Square::CARDINALITY)
    }

    #[quickcheck]
    fn repeated_counterclockwise_rotation_is_counterclockwise_rotation_with_repetitions(
        tile: Tile<Square>,
        repetitions: u8,
    ) -> bool {
        let repeated = (0..repetitions).fold(tile, |acc, _| acc.rotated_counterclockwise(1));
        repeated == tile.rotated_counterclockwise(repetitions as u64)
    }

    #[quickcheck]
    fn rotate_counterclockwise_preserves_number_of_connections(
        tile: Tile<Square>,
        rotations: u64,
    ) -> bool {
        tile.0.len() == tile.rotated_counterclockwise(rotations).0.len()
    }
}
