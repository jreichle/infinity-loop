use std::{fmt::Display, ops::Not};

use enumset::{EnumSet, EnumSetType};
use quickcheck::{Arbitrary, Gen};
use Square::{Down, Left, Right, Up};

use super::{cardinality::Cardinality, finite::Finite};

/// represents a direction vector for a tile connection
#[derive(Debug, Hash, EnumSetType)]
#[enumset(repr = "u32")]
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
    pub fn opposite(self) -> Self {
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

/// A tile is a 2D-shape with possible connections to orthogonal neighbor tiles.
/// The possible directions for connections correspond to the number of enum values in the EnumSetType
///
/// it is guaranteed to implement the [Copy] trait
///
/// Basic operations are checking present connections and rotating tiles
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile<A: EnumSetType>(pub EnumSet<A>);

impl<A: EnumSetType> Default for Tile<A> {
    fn default() -> Self {
        Self(EnumSet::default())
    }
}

impl<A: EnumSetType> Tile<A> {
    /// rotates the tile clockwise by one step
    ///
    /// each step is 360 degrees / number of enum values
    pub fn rotated_clockwise(&self, repetitions: u32) -> Self {
        let bit_rep = self.0.as_u32();
        let enum_values = EnumSet::<A>::bit_width();
        let repetitions = repetitions % enum_values;
        let rotated_bit_rep = (bit_rep << repetitions) | (bit_rep >> (enum_values - repetitions));
        Self(EnumSet::from_u32_truncated(rotated_bit_rep))
    }

    /// rotates the tile counterclockwise by one step
    ///
    /// each step is 360 degrees / number of enum values
    pub fn rotated_counterclockwise(&self, repetitions: u32) -> Self {
        let enum_values = EnumSet::<A>::bit_width();
        self.rotated_clockwise(enum_values - repetitions % enum_values)
    }
}

impl<A: EnumSetType> Not for Tile<A> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl<A: Cardinality + EnumSetType> Cardinality for EnumSet<A> {
    const CARDINALITY: u64 = 1 << A::CARDINALITY;
}

impl<A: Finite + EnumSetType> Finite for EnumSet<A> {
    fn index_to_enum(value: u64) -> Self {
        EnumSet::from_u64_truncated(value)
    }

    fn enum_to_index(&self) -> u64 {
        self.as_u64()
    }
}

impl<A: Cardinality + EnumSetType> Cardinality for Tile<A> {
    const CARDINALITY: u64 = EnumSet::<A>::CARDINALITY;
}

impl<A: Finite + EnumSetType> Finite for Tile<A> {
    fn index_to_enum(value: u64) -> Self {
        Self(EnumSet::index_to_enum(value))
    }

    fn enum_to_index(&self) -> u64 {
        self.0.enum_to_index()
    }
}

impl<A: 'static + EnumSetType> Arbitrary for Tile<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(EnumSet::from_u32_truncated(u32::arbitrary(g)))
    }
}

impl Display for Tile<Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.0.iter().map(|s| 1 << s.enum_to_index()).sum::<usize>();
        write!(f, "{}", Self::UNICODE_TILES[index])
    }
}

impl Tile<Square> {
    /// visualization for each configuration as UNICODE symbol
    const UNICODE_TILES: [&'static str; 16] = [
        " ", "╹", "╺", "┗", "╻", "┃", "┏", "┣", "╸", "┛", "━", "┻", "┓", "┫", "┳", "╋",
    ];

    pub fn get_value(&self) -> &str {
        let set = self.0;
        let mut index = 0;
        if set.contains(Up) {
            index += 1
        }
        if set.contains(Right) {
            index += 2
        }
        if set.contains(Down) {
            index += 4
        }
        if set.contains(Left) {
            index += 8
        }
        Self::UNICODE_TILES[index]
    }
}

#[cfg(test)]
mod tests {

    use crate::model::finite::Finite;

    use super::{Square, Tile};

    #[quickcheck]
    fn rotate_clockwise_4_times_is_identity(tile: Tile<Square>) -> bool {
        tile == tile.rotated_clockwise(4)
    }

    #[quickcheck]
    fn repeated_clockwise_rotation_is_clockwise_rotation_with_repetitions(
        tile: Tile<Square>,
        repetitions: u8,
    ) -> bool {
        let repeated = (0..repetitions).fold(tile, |acc, _| acc.rotated_clockwise(1));

        repeated == tile.rotated_clockwise(repetitions as u32)
    }

    #[quickcheck]
    fn rotate_clockwise_preserves_number_of_connections(
        tile: Tile<Square>,
        rotations: u32,
    ) -> bool {
        tile.0.len() == tile.rotated_clockwise(rotations).0.len()
    }

    #[quickcheck]
    fn rotate_counterclockwise_4_times_is_identity(tile: Tile<Square>) -> bool {
        tile == tile.rotated_counterclockwise(4)
    }

    #[quickcheck]
    fn repeated_counterclockwise_rotation_is_counterclockwise_rotation_with_repetitions(
        tile: Tile<Square>,
        repetitions: u8,
    ) -> bool {
        let repeated = (0..repetitions).fold(tile, |acc, _| acc.rotated_counterclockwise(1));
        repeated == tile.rotated_counterclockwise(repetitions as u32)
    }

    #[quickcheck]
    fn rotate_counterclockwise_preserves_number_of_connections(
        tile: Tile<Square>,
        rotations: u32,
    ) -> bool {
        tile.0.len() == tile.rotated_counterclockwise(rotations).0.len()
    }

    #[quickcheck]
    fn enumset_indexing_is_same_as_finite_indexing(tile: Tile<Square>) -> bool {
        tile.0.as_u64() == tile.0.into_iter().map(|s| 1 << s.enum_to_index()).sum()
    }
}
