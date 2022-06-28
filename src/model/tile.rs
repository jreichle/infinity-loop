use std::{collections::HashSet, fmt::Display};

use enumset::{EnumSet, EnumSetType};
use quickcheck::{Arbitrary, Gen};
use Square::{Down, Left, Right, Up};

/// represents a direction vector for a tile connection
#[derive(Hash, Debug, EnumSetType)]
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

/// equals powerset of full set
pub fn all_sets<A: EnumSetType>() -> HashSet<EnumSet<A>> {
    (0..(2 ^ EnumSet::<A>::variant_count()) - 1)
        .map(EnumSet::<A>::from_u32_truncated)
        .collect()
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

impl<A: 'static + EnumSetType> Arbitrary for Tile<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(EnumSet::from_u32_truncated(u32::arbitrary(g)))
    }
}

impl Display for Tile<Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "{}", Self::UNICODE_TILES[index])
    }
}

impl Tile<Square> {
    /// visualization for each configuration as UNICODE symbol
    const UNICODE_TILES: [&'static str; 16] = [
        "O", "╹", "╺", "┗", "╻", "┃", "┏", "┣", "╸", "┛", "━", "┻", "┓", "┫", "┳", "╋",
    ];
}

#[cfg(test)]
mod tests {

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
}
