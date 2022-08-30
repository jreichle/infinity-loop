use game::model::{grid::Grid, tile::Square, tile::Tile};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq)]
pub enum Screen {
    Level(Grid<Tile<Square>>),
    Overview,
    Title,
    Help,
    Credit,
    Editor,
    Visualizer,
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let res = match &self {
            Screen::Level(_) => "level",
            Screen::Overview => "overview",
            Screen::Title => "title",
            Screen::Help => "help",
            Screen::Credit => "credit",
            Screen::Editor => "editor",
            Screen::Visualizer => "visualizer",
        };
        write!(f, "{}", res)
    }
}

impl Screen {
    pub fn new(input: &str) -> Screen {
        match input {
            // TODO: try getting level from local_storage
            "level" => Screen::Level(Grid::EMPTY),
            "overview" => Screen::Overview,
            "title" => Screen::Title,
            "help" => Screen::Help,
            "credit" => Screen::Credit,
            "editor" => Screen::Editor,
            "visualizer" => Screen::Visualizer,
            _ => Screen::Title,
        }
    }
}
