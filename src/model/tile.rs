#![allow(dead_code)]

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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Tile(u8);


#[inline(always)]
fn test_bit(value: u8, index: u8) -> bool {
    value & (1 << index) != 0
}

impl Display for Tile {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Tile::ASCII_TILES[self.0 as usize])
    }
    
}

impl Tile {

    const BIT_SIZE: u8 = 4;
    const ASCII_TILES: [&'static str; 16] = [" ", "╹", "╺", "┗", "╻", "┃", "┏", "┣", "╸", "┛", "━", "┻", "┓", "┫", "┳", "╋"];
    
    /// creates new [Tile] from supplied value and panics if value > 15
    pub fn new(value: u8) -> Self {
        assert!(value < 16, "Tile::new: expected value between 0 and 15, but was {value}");
        Tile(value)
    }

    /// creates new [Tile] from supplied value and disregards higher bits
    pub fn truncate(value: u8) -> Self {
        Tile(value & 0xF)
    }

    pub fn has_connection_up(self) -> bool {
        test_bit(self.0, 3)
    }

    pub fn has_connection_right(self) -> bool {
        test_bit(self.0, 2)
    }

    pub fn has_connection_down(self) -> bool {
        test_bit(self.0, 1)
    }

    pub fn has_connection_left(self) -> bool {
        test_bit(self.0, 0)
    }

    // rotates the 4 least significant bits
    pub fn rotate_clockwise(self, number: u8) -> Self {
        Tile::truncate((self.0 << number) | (self.0 >> (Tile::BIT_SIZE - number)))
    }

    pub fn rotate_counterclockwise(self, number: u8) -> Self {
        Tile::truncate((self.0 >> number) | (self.0 << (Tile::BIT_SIZE - number)))
    }

}