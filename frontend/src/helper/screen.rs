use game::model::{grid::Grid, tile::Square, tile::Tile};
use std::fmt::{Display, Formatter, Result};

/// can be used to indicate which screen needs to be shown
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
