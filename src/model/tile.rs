use std::fmt::Display;

use enumset::{EnumSet, EnumSetType};
use quickcheck::{Arbitrary, Gen};
use Square::{Down, Left, Right, Up};

/// represents a direction vector for a tile connection
#[derive(Hash, Debug, EnumSetType)]
#[enumset(repr = "u32")]
pub enum Square {
    /// Coordinate(0, 1)
    Up,
    /// Coordinate(1, 0)
    Right,
    /// Coordinate(0, -1)
    Down,
    /// Coordinate(-1, 0)
    Left,
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

// impl Not for Square {
//     type Output = Self;
//
//     fn not(self) -> Self::Output {
//         match self {
//             Up => Down,
//             Right => Left,
//             Down => Up,
//             Left => Right,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile<A: EnumSetType>(pub EnumSet<A>);

impl<A: EnumSetType> Default for Tile<A> {
    fn default() -> Self {
        Self(EnumSet::default())
    }
}

impl<A: EnumSetType> Tile<A> {
    pub fn rotated_clockwise(&self, repetitions: u32) -> Self {
        let bit_rep = self.0.as_u32();
        let repetitions = repetitions % 4;
        let rotated_bit_rep =
            (bit_rep << repetitions) | (bit_rep >> (EnumSet::<A>::bit_width() - repetitions));
        Self(EnumSet::from_u32_truncated(rotated_bit_rep))
    }

    pub fn rotated_counterclockwise(&self, repetitions: u32) -> Self {
        self.rotated_clockwise(4 - repetitions % 4)
    }
}

impl<A: 'static + EnumSetType> Arbitrary for Tile<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(EnumSet::from_u32_truncated(u32::arbitrary(g)))
    }
}

impl Display for Tile<Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::UNICODE_TILES[self.0.as_usize()])
    }
}

impl Tile<Square> {
    const UNICODE_TILES: [&'static str; 16] = [
        " ", "╹", "╺", "┗", "╻", "┃", "┏", "┣", "╸", "┛", "━", "┻", "┓", "┫", "┳", "╋",
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
