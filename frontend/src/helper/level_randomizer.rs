use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use game::model::grid::Grid;
use game::model::tile::{Square, Tile};

use rand::Rng;

/// randomizes the given level until it is not in a solved state anymore
pub fn randomize_level(mut level: Grid<Tile<Square>>) -> Grid<Tile<Square>> {
    let dimension = level.dimensions();
    let mut is_solved = true;
    while is_solved {
        for row in 0..dimension.row {
            for col in 0..dimension.column {
                let num = rand::thread_rng().gen_range(0..3);
                level = level
                    .rotate_clockwise_n_times(Coordinate::new(row as isize, col as isize), num)
                    .unwrap();
            }
        }
        is_solved = level.is_solved();
    }
    level
}
