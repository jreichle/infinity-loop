use std::{
    fmt::Display,
    ops::{Neg, Not},
};

use quickcheck::{Arbitrary, Gen};
use Square::{Down, Left, Right, Up};

use super::{cardinality::Cardinality, enumset::EnumSet, finite::Finite};

/// Represents a direction for a tile connection
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    /// upwards direction or [`Coordinate::new(-1, 0)`][model::Coordinate::new]
    Up,
    /// rightwards direction or [`Coordinate::new(0, 1)`](model::Coordinate::new)
    Right,
    /// downwards direction or [`Coordinate::new(1, 0)`](model::Coordinate::new)
    Down,
    /// leftwards direction or [`Coordinate::new(0, -1)`](model::Coordinate::new)
    Left,
}

/// with directions as aliases for delta coordinates the opposite operation is closer to numerical negation than logical complement
impl Neg for Square {
    type Output = Square;

    fn neg(self) -> Self::Output {
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
    fn unchecked_index_to_enum(value: u64) -> Self {
        match value % Self::CARDINALITY {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            _ => Self::Left,
        }
    }

    fn enum_to_index(&self) -> u64 {
        // *self as u64; may be faster, but dependent on declaration order
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
        Self::NO_CONNECTIONS
    }
}

impl<A> Tile<A> {
    pub const NO_CONNECTIONS: Tile<A> = Self(EnumSet::EMPTY);
}

impl<A: Cardinality> Tile<A> {
    pub const ALL_CONNECTIONS: Tile<A> = Self(EnumSet::FULL);
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
        Self(EnumSet::unchecked_index_to_enum(rotated_bit_rep))
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
    fn unchecked_index_to_enum(value: u64) -> Self {
        Self(EnumSet::unchecked_index_to_enum(value))
    }

    fn enum_to_index(&self) -> u64 {
        self.0.enum_to_index()
    }
}

impl<A: 'static + Clone + Finite> Arbitrary for Tile<A> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(EnumSet::unchecked_index_to_enum(u64::arbitrary(g)))
    }
}

impl Display for Tile<Square> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::UNICODE_TILES[self.enum_to_index() as usize])
    }
}

impl Tile<Square> {
    /// visualization for each tile state as UNICODE character
    const UNICODE_TILES: [char; 16] = [
        ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
    ];
}

#[macro_export]
macro_rules! tile {
    ( $( $e:expr ), * ) => {{
        Tile(enumset!( $( $e ),* ))
    }};
}

#[cfg(test)]
mod tests {

    use crate::model::{cardinality::Cardinality, finite::Finite};

    use super::{Square, Tile};

    /// not necessary, but desirable
    #[quickcheck]
    fn square_finite_defines_order_isomorphism(s1: Square, s2: Square) -> bool {
        (s1 <= s2) == (s1.enum_to_index() <= s2.enum_to_index())
    }

    /// not necessary, but desirable
    #[quickcheck]
    fn tile_finite_defines_order_isomorphism(t1: Tile<Square>, t2: Tile<Square>) -> bool {
        (t1 <= t2) == (t1.enum_to_index() <= t2.enum_to_index())
    }

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
