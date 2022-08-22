use yew::prelude::*;
use yew::{html, Callback, Html};

use game::model::gameboard::GameBoard;
use game::model::tile::Square;
use game::model::{grid::Grid, tile::Tile};

// needs to contain
// - hook to level -> if level stream works out maybe just level stream and -> next
// - levelstream
//      - hook to level stream
//          - overview as to take multiple level
//          - if level in middle is picked -> level stream is missing levels in between
//          - maybe: if level in between was picked -> remaining into iterator -> chain
//      - int with picked level
//          -> level compoenent can setup level stream

#[derive(Properties, PartialEq, Clone)]
pub struct OverviewComponentProps {
    level: UseStateHandle<Grid<Tile<Square>>>,
}

#[function_component(OverviewComponent)]
pub fn overview_component(props: &OverviewComponentProps) -> Html {
    // grid of levels -> return correct level
    html! {
        <>
            <div>
                {"Overview"}
            </div>
        </>
    }
}
