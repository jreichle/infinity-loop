use game::generator::levelstream::{level_stream, LevelProperty};
use game::model::coordinate::Coordinate;
use game::model::gameboard::GameBoard;
use yew::prelude::*;

//Create the main app that will load all other Components
pub struct App {}

//Message enum that is used for managing the life cycle of Components
pub enum Msg {}

//Implement the Component interface
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //Creates The HTML that will show up in the browser.
        let property = LevelProperty {
            dimension: 5.into(),
        };

        if let Some(get_levels) = level_stream(property).next() {
            let level = &get_levels(1);
            let pos_to_tile = level.serialize_board();
            let rows = extract_res_or_default(level.rows().try_into(), 0);
            let cols = extract_res_or_default(level.columns().try_into(), 0);
            let mut current_row: isize = 0;
            let mut current_col: isize = 0;

            log::info!("\n {}", level);
            while current_row < rows {
                while current_col < cols {
                    log::info!(
                        "row: {}, col: {}, tile: {:?}",
                        current_row,
                        current_col,
                        pos_to_tile.get(&Coordinate {
                            row: current_row,
                            column: current_col
                        })
                    );
                    current_col += 1;
                }
                current_row += 1;
            }
            html! {
                <div> {"working"} </div>
            }
        } else {
            html! {
                <div>
                    {"level not working"}
                    <div></div>
                    <button> {"fuck"} </button>
                </div>
            }
        }
    }
}

fn extract_res_or_default<T, E>(res: Result<T, E>, default: T) -> T {
    match res {
        Ok(ret) => ret,
        Err(err) => default,
    }
}

pub fn main() {
    //Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    //Start the yew framework
    yew::start_app::<App>();
}
