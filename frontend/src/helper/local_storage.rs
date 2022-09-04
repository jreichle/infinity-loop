use super::screen::Screen;

use game::model::grid::Grid;
use game::model::parser::{parse_level, unicode_to_tile};
use game::model::tile::{Square, Tile};
use yew::prelude::*;

pub const CURRENT_SCREEN: &str = "screen";
pub const CURRENT_LEVEL: &str = "level";
pub const SAVED_LEVEL: &str = "saved level";
pub const PREVIEW_LEVELS: &str = "preview levels";

/// saves the number of levels that need to be loaded for the preview to the local storage
pub fn save_preview_level_count(nr_levels: usize) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(PREVIEW_LEVELS, nr_levels.to_string().as_str())
        .unwrap();
}

/// retrieves the number of levels that need to be loaded for the preview from the local storage
pub fn retrieve_preview_level_count() -> usize {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(nr_levels)) = local_storage.get_item(PREVIEW_LEVELS) {
        nr_levels.parse().unwrap()
    } else {
        20
    }
}

/// saves a level from the editor to the local storage that can later be retrieved
pub fn save_editor_level(grid: &Grid<Tile<Square>>) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(SAVED_LEVEL, grid.to_string().as_str())
        .unwrap();
}

/// retrieves a previously saved editor level from local storage
pub fn retrieve_editor_level() -> Grid<Tile<Square>> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(level)) = local_storage.get_item(SAVED_LEVEL) {
        parse_level(level.as_str(), unicode_to_tile).unwrap()
    } else {
        Grid::EMPTY
    }
}

/// saves a playing level to the local storage that can later be retrieved
pub fn save_level(grid: &Grid<Tile<Square>>) {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    local_storage
        .set_item(CURRENT_LEVEL, &grid.to_string())
        .unwrap();
}

/// retrieves a previously saved playing level from local storage
fn retrieve_level() -> Grid<Tile<Square>> {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(level)) = local_storage.get_item(CURRENT_LEVEL) {
        parse_level(level.as_str(), unicode_to_tile).unwrap()
    } else {
        Grid::EMPTY
    }
}

/// changes the screen of the app and saves it to the local storage
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

/// retrieves a previously saved screen from local storage
///
/// default screen is the title screen, in case retrieval fails
pub fn retrieve_screen() -> Screen {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(saved_screen)) = local_storage.get_item(CURRENT_SCREEN) {
        log::info!("retrieved old screen: {}", saved_screen);
        match saved_screen.as_str() {
            "level" => Screen::Level(retrieve_level()),
            "overview" => Screen::Overview,
            "title" => Screen::Title,
            "help" => Screen::Help,
            "credit" => Screen::Credit,
            "editor" => Screen::Editor,
            "visualizer" => Screen::Visualizer,
            _ => Screen::Title,
        }
    } else {
        log::info!("default screen: title");
        Screen::Title
    }
}
