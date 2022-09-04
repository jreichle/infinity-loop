#![allow(dead_code)]

use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

use crate::helper::level_randomizer::randomize_level;
use crate::helper::local_storage::save_level;
use wasm_bindgen::{prelude::*, JsCast};

use game::generator::wfc::WfcSettings;
use game::model::hint::{generate_solving_trace, get_hint};
use game::model::{
    coordinate::Coordinate,
    fastgen::generate,
    gameboard::GameBoard,
    grid::Grid,
    tile::{Square, Tile},
};

// reducer's action
pub enum BoardAction {
    TurnCell(Coordinate<isize>),
    ReplaceGrid(Grid<Tile<Square>>),
    NextLevel,
    GetHint,
    SolveLevel,

    // Editor actions
    ChangeTileShape(Coordinate<isize>),
    ChangeSize(Coordinate<usize>),
    GenerateFastGen,
    GenerateWFC,
    ShuffleTileRotations,
    ClearGrid,
}

// reducer's state
// is a functor
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Level<A> {
    pub id: usize,
    pub data: A,
}

impl<A> Level<A> {
    fn new(id: usize, data: A) -> Self {
        Level { id, data }
    }
}

#[wasm_bindgen]
pub fn highlight_cells(row: usize, column: usize) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let cell = document
        .get_element_by_id(&format!("cell-r-{}-c-{}", row, column))
        .unwrap();

    let class_names = cell.get_attribute("class").unwrap();
    cell.set_class_name(&format!("{} {}", class_names.clone(), "cell-hint-highlight"));
    let hl = Closure::<dyn Fn()>::new(move || {
        cell.set_class_name(&class_names);
    });

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(hl.as_ref().unchecked_ref(), 500)
        .ok();
    hl.forget();
}

impl Reducible for Level<Grid<Tile<Square>>> {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            BoardAction::TurnCell(index) => {
                Level::new(self.id, self.data.rotate_clockwise(index).unwrap()).into()
            }
            BoardAction::ReplaceGrid(grid) => {
                Level::new(self.id, grid).into()
            }
            BoardAction::NextLevel => {
                let data = randomize_level(generate(
                    self.data.dimensions() + 1,
                    self.id as u64,
                ));
                save_level(&data);
                Level::new(self.id + 1, data).into()

            }
            BoardAction::GetHint => {
                let trace = generate_solving_trace(&self.data);
                log::info!("trace: {:?}", trace);
                if let Ok(coordinate) = get_hint(&self.data, trace) {
                    highlight_cells(
                        coordinate.row as usize,
                        coordinate.column as usize,
                    );
                    log::info!("Highlighting: {}", coordinate);
                }
                self
            }
            BoardAction::SolveLevel => {
                match self.data.solve().next() {
                    None => self,
                    Some(solution) => {
                        log::info!("solved level:\n{solution}");
                        save_level(&solution);
                        Level::new(self.id, solution).into()
                    }
                }
            }

            // Editor actions
            BoardAction::ChangeTileShape(index) => {
                log::info!("Change tile shape");
                Level::new(self.id, self.data.change_tile_shape(index).unwrap()).into()
            }
            BoardAction::ChangeSize(size) => {
                Level::new(self.id, generate(size, rand::thread_rng().gen_range(0..10000))).into()
            }
            BoardAction::GenerateFastGen => {
                let data = generate(self.data.dimensions(), rand::thread_rng().gen_range(0..10000));
                log::info!("Generated grid\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::GenerateWFC => {
                let wfc_settings = WfcSettings::with_all_tiles(
                    self.data.columns(),
                    self.data.rows(),
                );
                let data = retry_until_ok(wfc_settings, WfcSettings::generate);

                log::info!("Generated grid\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::ShuffleTileRotations => {
                let data = randomize_level(self.data.clone());
                save_level(&data);
                log::info!("Tile rotations shuffled\n{data}");
                Level::new(self.id, data).into()
            }
            BoardAction::ClearGrid => {
                let data = Grid::filled_with(
                    self.data.dimensions(),
                    Tile::NO_CONNECTIONS,
                );
                Level::new(self.id, data).into()
            }
        }
    }
}

impl Level<Grid<Tile<Square>>> {
    pub fn set_size(dimensions: Coordinate<usize>) -> impl Fn() -> Level<Grid<Tile<Square>>> {
        move || Level {
            id: 1,
            data: generate(dimensions, 1),
        }
    }

    pub fn set_grid(grid: Grid<Tile<Square>>) -> impl Fn() -> Level<Grid<Tile<Square>>> {
        move || Level {
            id: 1,
            data: grid.clone(),
        }
    }
}

/// Retries an impure computation until it succeeds
fn retry_until_ok<A, B, E, F: Fn(&A) -> Result<B, E>>(initial: A, computation: F) -> B {
    loop {
        match computation(&initial) {
            Err(_) => continue,
            Ok(value) => return value
        }
    }
}
