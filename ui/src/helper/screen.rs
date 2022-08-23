use game::model::{grid::Grid, tile::Tile, tile::Square};

#[derive(PartialEq)]
pub enum Screen {
    Level(Grid<Tile<Square>>),
    Overview,
    Title,
    Editor,
}
