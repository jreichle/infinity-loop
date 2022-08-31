use super::screen::Screen;

use game::model::grid::Grid;
use game::model::testlevel::{parse_level, unicode_to_tile};
use game::model::tile::{Square, Tile};
use yew::prelude::*;

pub const CURRENT_SCREEN: &str = "screen";
pub const CURRENT_LEVEL: &str = "level";
pub const SAVED_LEVEL: &str = "saved level";

pub fn save_editor_level(grid: &Grid<Tile<Square>>) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(SAVED_LEVEL, grid.to_string().as_str())
        .unwrap();
}

pub fn retrieve_editor_level() -> Grid<Tile<Square>> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(level)) = local_storage.get_item(SAVED_LEVEL) {
        parse_level(&level.as_str(), unicode_to_tile).unwrap()
    } else {
        Grid::EMPTY
    }
}

pub fn save_level(grid: &Grid<Tile<Square>>) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(CURRENT_LEVEL, &grid.to_string())
        .unwrap();
}

fn retrieve_level() -> Grid<Tile<Square>> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(level)) = local_storage.get_item(CURRENT_LEVEL) {
        parse_level(&level.as_str(), unicode_to_tile).unwrap()
    } else {
        Grid::EMPTY
    }
}

pub fn change_screen(screen: UseStateHandle<Screen>, to_screen: Screen) {
    save_screen(&to_screen);
    screen.set(to_screen);
}

fn save_screen(saving_screen: &Screen) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(CURRENT_SCREEN, &saving_screen.to_string())
        .unwrap();
    if let Screen::Level(level) = saving_screen {
        save_level(level);
    }
    log::info!("saved screen: {}", &saving_screen.to_string().as_str());
}

pub fn retrieve_screen() -> Screen {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(saved_screen)) = local_storage.get_item(CURRENT_SCREEN) {
        log::info!("retrieved old screen: {}", saved_screen);
        let screen = Screen::new(saved_screen.as_str());
        if let Screen::Level(_) = screen {
            Screen::Level(retrieve_level())
        } else {
            screen
        }
    } else {
        log::info!("default screen: title");
        Screen::Title
    }
}
