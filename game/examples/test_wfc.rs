#![allow(dead_code, unused_imports)]
use game::core::enumset::EnumSet;
use game::generator::wfc::{self, WfcGenerator};
use game::model::tile::{
    Square,
    Square::{Down, Left, Right, Up},
    Tile,
};
use game::solver::propagationsolver::SentinelGrid;
use game::{enumset, tile};

use std::process::Command;

static WIDTH: usize = 35;
static HEIGHT: usize = 35;
static PASS_LIMIT: usize = 40000;
static PROP_LIMIT: usize = 1000;

fn wfc_test_full_set() -> bool {
    let available_tiles = EnumSet::FULL;
    wfc_test(WIDTH, HEIGHT, available_tiles, PASS_LIMIT, PROP_LIMIT)
}

fn wfc_test_part_set() -> bool {
    let available_tiles = enumset!(
        Tile::NO_CONNECTIONS,
        tile!(Right, Down),
        tile!(Up, Right),
        tile!(Down, Left),
        tile!(Up, Left)
    );
    wfc_test(WIDTH, HEIGHT, available_tiles, PASS_LIMIT, PROP_LIMIT)
}

fn wfc_test(
    width: usize,
    height: usize,
    available_tiles: EnumSet<Tile<Square>>,
    pass_limit: usize,
    prop_limit: usize,
) -> bool {
    let wfc_generator = WfcGenerator::new(width, height, available_tiles, pass_limit, prop_limit);
    let mut generation_result = wfc_generator.generate();

    while let Err(_) = generation_result {
        generation_result = wfc_generator.generate();
    }

    match generation_result {
        Ok(board) => {
            println!("Final board: ");
            println!("{}", format!("{}", board.to_string()));
            // if &board.solve().count() >= &1 {
            //     println!("[O] level solvable.");
            //     println!("----------------------------");
            //     return true;
            // } else {
            //     println!("[X] level not solvable.");
            //     println!("----------------------------");
            //     return false;
            // }
            true
        }
        Err(msg) => {
            println!("{}", msg);
            return false;
        }
    }
}

// fn extract_uncomplete_board(&board: SentinelGrid<EnumSet<Tile<Square>>) -> Grid<Tile<Square>> {

// }

fn wfc_test_step() {
    let available_tiles = EnumSet::FULL;
    wfc_step_by_step(WIDTH, HEIGHT, available_tiles, PASS_LIMIT, PROP_LIMIT)
}

fn wfc_step_by_step(
    width: usize,
    height: usize,
    available_tiles: EnumSet<Tile<Square>>,
    pass_limit: usize,
    prop_limit: usize,
) {
    let wfc_generator = WfcGenerator::new(width, height, available_tiles, pass_limit, prop_limit);
    let (mut board, mut weights) = wfc_generator.init_board();

    let mut passes: usize = 0;
    loop {
        // let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        (board, weights) = wfc_generator.iteration_step(board, weights);

        // WfcGenerator::print_map(&board);
        let grid = WfcGenerator::extract_grid(&board);
        println!("{}", grid.to_string());

        passes += 1;

        if WfcGenerator::is_all_collapsed(&board) || passes >= pass_limit {
            break;
        }
    }

    let _final_grid = board.extract_if_collapsed();
}

fn main() {
    for _ in 0..2 {
        wfc_test_full_set();
        wfc_test_part_set();
    }

    // wfc_test_step()
}
