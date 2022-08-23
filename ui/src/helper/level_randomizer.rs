use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use rand::Rng;

pub fn randomize_level(mut level: Grid<Tile<Square>>) -> Grid<Tile<Square>> {
    let dimension = level.dimensions();
    while level.is_solved() {
        for row in 0..dimension.row {
            for col in 0..dimension.column {
                let num = rand::thread_rng().gen_range(0..3);
                for _ in 0..num {
                    level = level
                        .rotate_clockwise(Coordinate::new(row as isize, col as isize))
                        .unwrap();
                }
            }
        }
    }
    level
}
