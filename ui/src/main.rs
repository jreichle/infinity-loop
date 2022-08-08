use yew::prelude::*;
use yew::{html, ChildrenWithProps, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct CellUnitProp {
    value: usize,
}

pub enum CellUnitMsg {
    TurnTile,
}

pub struct MapCellComponent {
    value: usize,
    // todo: add coordinate
    angel: usize,
}

impl Component for MapCellComponent {
    type Message = CellUnitMsg;
    type Properties = CellUnitProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: _ctx.props().value.clone(),
            angel: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CellUnitMsg::TurnTile => {
                log::info!("Turn tile");
                self.angel = (self.angel + 90) % 360;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let img_path = vec![
            "data/tiles/0.png",
            "data/tiles/1.png",
            "data/tiles/2.png",
            "data/tiles/3.png",
            "data/tiles/4.png",
            "data/tiles/5.png",
        ];
        log::info!("tile: {}", img_path[self.value.clone()]);
        let link = ctx.link();
        html! {
            <div class="cell">
                <img src={ img_path[self.value.clone()] }
                    onclick={link.callback(|_| CellUnitMsg::TurnTile)}
                    style={format!("{}{}{}","transform:rotate(", self.angel.clone().to_string(), "deg);")}
                />
            </div>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct RowProp {
    pub children: ChildrenWithProps<MapCellComponent>,
}

pub struct MapRowComponent;

impl Component for MapRowComponent {
    type Message = ();
    type Properties = RowProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="cell-row">
                { ctx.props().children.clone() }
            </div>
        }
    }
}

struct MapTableComponent {
    // width: usize,
    // height: usize,
    children: ChildrenWithProps<MapRowComponent>,
}

#[derive(PartialEq, Properties)]
struct MapProp {
    id: String,
    children: ChildrenWithProps<MapRowComponent>,
}

pub enum MapMsg {
    CheckValid,
    GetSolution,
    NextLevel,
}

impl Component for MapTableComponent {
    type Message = MapMsg;
    type Properties = MapProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            children: _ctx.props().children.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MapMsg::CheckValid => {
                log::info!("-> Check if valid solution");
                false
            }
            MapMsg::GetSolution => {
                log::info!("-> Get solution.");
                true
            }
            MapMsg::NextLevel => {
                log::info!("-> Get next level.");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        log::info!("{:?}", self.children.clone());

        html! {
            <>
                <div id="container">
                    <div id={ctx.props().id.clone()} class="game-board">
                        { self.children.clone() }
                    </div>
                    <div id="controller">
                        <button onclick={link.callback(|_| MapMsg::CheckValid)} >{"check"}</button>
                        <button onclick={link.callback(|_| MapMsg::GetSolution)} >{"solve"}</button>
                        <button onclick={link.callback(|_| MapMsg::NextLevel)} >{"next"}</button>
                    </div>
                </div>
            </>
        }
    }
}

fn _get_index(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' => 0,
        '-' => 1,
        'I' => 2,
        'L' => 3,
        'T' => 4,
        '+' => 5,
        _ => 0,
    }
}

#[function_component(App)]
fn app() -> Html {
    // let leveldata: &str = "LLLTL\nTTT+T\nLL LT\nLTLIT\nLTILT\n-TTL-";
    // let level = leveldata.lines().collect::<Vec<_>>();

    // let map = level.into_iter().map( | row | {
    //     let cell_row = row.chars().map( | cell | {
    //         html_nested! {
    //             <MapCellComponent value={get_index(cell.clone())}/>
    //         }
    //     });

    //     html! {
    //         <MapRowComponent>
    //         {
    //             for cell_row
    //         }
    //         </MapRowComponent>
    //     }
    // });

    html! {
        <>
            <div id="title">{"Rusty infinity loop!"}</div>
            <MapTableComponent id="game-1">
                // { for map }
                <MapRowComponent>
                    <MapCellComponent value=3 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=4 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=3 />
                </MapRowComponent>

                <MapRowComponent>
                    <MapCellComponent value=2 />
                    <MapCellComponent value=1 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=1 />
                    <MapCellComponent value=2 />
                </MapRowComponent>

                <MapRowComponent>
                    <MapCellComponent value=2 />
                    <MapCellComponent value=4 />
                    <MapCellComponent value=5 />
                    <MapCellComponent value=4 />
                    <MapCellComponent value=2 />
                </MapRowComponent>

                <MapRowComponent>
                    <MapCellComponent value=2 />
                    <MapCellComponent value=1 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=1 />
                    <MapCellComponent value=2 />
                </MapRowComponent>

                <MapRowComponent>
                    <MapCellComponent value=3 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=4 />
                    <MapCellComponent value=2 />
                    <MapCellComponent value=3 />
                </MapRowComponent>
            </MapTableComponent>

        </>
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    log::info!("frontend starting...");
}
