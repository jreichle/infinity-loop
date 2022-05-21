use quickcheck::{Arbitrary, Gen};

use super::{coordinate::Coordinate, tile::Tile};
use std::collections::HashMap;

/// naive implementation, API is bound to change
/// 
/// invariant through game design: board forms a recangle completely filled with [Tile]s
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Board(HashMap<Coordinate<u32>, Tile>);


impl Arbitrary for Board {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(HashMap::arbitrary(g))
    }
}