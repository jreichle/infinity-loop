use game::model::{grid::Grid, tile::Square, tile::Tile};

#[derive(PartialEq)]
pub enum Screen {
    Level(Grid<Tile<Square>>),
    Overview,
    Title,
    Editor,
}
