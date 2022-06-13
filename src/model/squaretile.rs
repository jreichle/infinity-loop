use quickcheck::{Arbitrary, Gen};
use std::fmt::Display;

/// A tile is a 2D-square with possible connections to orthogonal neighbors denoted by lines pointing outwards from the center.
/// It is encoded with the 4 least significant bits as 0b000_URDL with 0 representing no connection
/// in that particular direction and 1 representing a present one.
/// The 4 possible directions are in clockwise direction: Up, Right, Down, Left
///
/// * Basic operations are checking present connections and rotating tiles
/// * invariant: values are between 0 and 15
///
/// implementation note: would ideally use u4
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct SquareTile(u8);

#[inline(always)]
const fn test_bit(value: u8, index: u8) -> bool {
    value & (1 << index) != 0
}

impl Arbitrary for SquareTile {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::truncate(u8::arbitrary(g))
    }
}

impl Display for SquareTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::UNICODE_TILES[self.0 as usize])
    }
}

impl SquareTile {
    /// number of bits used to save a single [Tile] configuration
    const BIT_SIZE: u8 = 4;

    /// bit mask for all used bits
    const USED_BITS: u8 = (1 << Self::BIT_SIZE) - 1;

    /// visualization for each configuration as UNICODE symbol
    const UNICODE_TILES: [&'static str; 16] = [
        " ", "╹", "╺", "┗", "╻", "┃", "┏", "┣", "╸", "┛", "━", "┻", "┓", "┫", "┳", "╋",
    ];

    /// creates new [Tile] from supplied value and panics if value > 15
    pub fn new(value: u8) -> Self {
        assert!(
            value <= 0xF,
            "Tile::new: expected value between 0 and 15, but was {value}"
        );
        Self(value)
    }

    /// creates new [Tile] from supplied value and disregards higher bits
    pub const fn truncate(value: u8) -> Self {
        Self(value & Self::USED_BITS)
    }

    pub const fn has_connection_up(&self) -> bool {
        test_bit(self.0, 3)
    }

    pub const fn has_connection_right(&self) -> bool {
        test_bit(self.0, 2)
    }

    pub const fn has_connection_down(&self) -> bool {
        test_bit(self.0, 1)
    }

    pub const fn has_connection_left(&self) -> bool {
        test_bit(self.0, 0)
    }

    // rotates the 4 least significant bits
    // 0100 -> 1000
    pub fn rotated_clockwise(&self, repetitions: u8) -> Self {
        let repetitions = repetitions % 4;
        let rotated = (self.0 << repetitions) | (self.0 >> (Self::BIT_SIZE - repetitions));
        Self::truncate(rotated)
    }

    pub fn rotated_counterclockwise(&self, repetitions: u8) -> Self {
        self.rotated_clockwise(4 - repetitions % 4)
    }
}

#[cfg(test)]
mod tests {

    use super::SquareTile;

    #[quickcheck]
    fn tile_construction_and_deconstruction(value: u8) -> bool {
        match value {
            0x0..=0xF => SquareTile::new(value).0 == value,
            _ => std::panic::catch_unwind(|| SquareTile::new(value)).is_err(),
        }
    }

    #[quickcheck]
    fn rotate_clockwise_4_times_is_identity(tile: SquareTile) -> bool {
        tile == tile.rotated_clockwise(4)
    }

    #[quickcheck]
    fn repeated_clockwise_rotation_is_clockwise_rotation_with_repetitions(
        tile: SquareTile,
        repetitions: u8,
    ) -> bool {
        let successive = (0..repetitions).fold(tile, |acc, _| acc.rotated_clockwise(1));
        successive == tile.rotated_clockwise(repetitions)
    }

    #[quickcheck]
    fn rotate_clockwise_preserves_number_of_connections(tile: SquareTile, rotations: u8) -> bool {
        tile.0.count_ones() == tile.rotated_clockwise(rotations).0.count_ones()
    }

    #[quickcheck]
    fn rotate_counterclockwise_4_times_is_identity(tile: SquareTile) -> bool {
        tile == tile.rotated_counterclockwise(4)
    }

    fn repeated_counterclockwise_rotation_is_counterclockwise_rotation_with_repetitions(
        tile: SquareTile,
        repetitions: u8,
    ) -> bool {
        let successive = (0..repetitions).fold(tile, |acc, _| acc.rotated_clockwise(1));
        successive == tile.rotated_counterclockwise(repetitions)
    }

    #[quickcheck]
    fn rotate_counterclockwise_preserves_number_of_connections(tile: SquareTile, rotations: u8) -> bool {
        tile.0.count_ones() == tile.rotated_counterclockwise(rotations).0.count_ones()
    }
}
