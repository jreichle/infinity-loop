use super::accesserror::AccessError;

/// complete interface to interact with game
// sending information to the view:
//  1. return changed grid parts as list of actions
//  2. send whole grid as `HasMap Coordinate Tile`
pub trait GameBoard
where
    Self: Sized,
{
    /// defines indexing system for the given [Model]
    type Index;

    /// performs single clockwise rotation
    ///
    /// - returns changed model in case of success
    /// - returns [AccessError] otherwise
    fn rotate_clockwise(&self, index: Self::Index) -> Result<Self, AccessError>;

    /// performs single counterclockwise rotation
    ///
    /// - returns changed model in case of success
    /// - returns [AccessError] otherwise
    fn rotate_counterclockwise(&self, index: Self::Index) -> Result<Self, AccessError>;

    /// queries if gameboard is solved
    fn is_solved(&self) -> bool;
}
