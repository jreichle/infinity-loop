use yew::prelude::*;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct CellComponemtProps {
    pub value: usize,
}

#[function_component(CellComponemt)]
pub fn cellComponemt(props: &CellComponemtProps) -> Html {
    let img_path = vec![
        "data/tiles/0.png",
        "data/tiles/1.png",
        "data/tiles/2.png",
        "data/tiles/3.png",
        "data/tiles/4.png",
        "data/tiles/5.png",
    ];

    html! {
        <div class="cell">
        <img src={ img_path[props.value] }
        // onclick={link.callback(|_| CellUnitMsg::TurnTile)}
        // style={format!("{}{}{}","transform:rotate(", self.angel.clone().to_string(), "deg);")}
        />

        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponemtProps {
    pub children: Vec<CellComponemtProps>,
}

#[function_component(RowComponemt)]
pub fn rowComponemt(props: &RowComponemtProps) -> Html {
    html! {
        <div class="cell-row">
        {
            props.children.iter().enumerate().map(| (i, child) | {
                html! { <CellComponemt key={i} ..child.clone() /> }
            }).collect::<Html>()
        }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MapComponemtProps {
    pub id: usize,
    pub children: Vec<RowComponemtProps>,
}

#[function_component(MapComponemt)]
pub fn mapComponemt(props: &MapComponemtProps) -> Html {
    html! {
        <>
            <div class="game-board">
            {
                props.children.iter().enumerate().map(| (i, child) | {
                    html!{ <RowComponemt key={i} ..child.clone() /> }
                }).collect::<Html>()
            }
            </div>
            <div id="controller">
                <button 
                    // onclick={link.callback(|_| MapMsg::CheckValid)} 
                    >{"check"}</button>
                <button
                    // onclick={link.callback(|_| MapMsg::GetSolution)} 
                    >{"solve"}</button>
                <button 
                    // onclick={link.callback(|_| MapMsg::NextLevel)} 
                    >{"next"}</button>
            </div>
        </>
    }
}

pub fn get_index(cell_symbol: char) -> usize {
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