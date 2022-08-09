use yew::prelude::*;
use yew::{html, Html, Properties, Callback, use_state};

#[derive(Properties, PartialEq, Clone)]
pub struct CellComponentProps {
    coordinate: (usize, usize),
    value: usize,
}

#[function_component(CellComponent)]
pub fn cellComponent(props: &CellComponentProps) -> Html {
    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    let angle = use_state(|| 0_usize);
    let onclick: Callback<MouseEvent> = {
        log::info!("Tile with coordinate {:?} has been clicked.", props.coordinate);
        let angle = angle.clone();
        Callback::from(move |_| angle.set((*angle + 90) % 360))
    };

    html! {
        <div class="cell">
            <img src={ img_path[props.value] }
                onclick={onclick}
                style={format!("{}{}{}","transform:rotate(", *angle, "deg);")}
            />
        </div>
    }

}

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponentProps {
    row_count: usize,
    children: Vec<CellComponentProps>,
}

#[function_component(RowComponent)]
pub fn rowComponent(props: &RowComponentProps) -> Html {
    html! {
        <div class="cell-row">
        {
            props.children.iter().enumerate().map(| (i, child) | {
                html! { <CellComponent key={i} ..child.clone() /> }
            }).collect::<Html>()
        }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct MapComponentProps {
    // id: usize,
    pub level_data: String,
}

#[function_component(MapComponent)]
pub fn mapComponent(props: &MapComponentProps) -> Html {
    let level = parse_level(props.level_data.clone());

    let check_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            // let level = level.clone();
            // level.set(props_new.children.clone());
            log::info!("[Button click] Check.");
        })
    };

    let solve_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Solve.")
        })
    };

    let next_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("[Button click] Next.")
        })
    };

    html! {
        <>
            <div class="game-board">
            {
                level.iter().enumerate().map(| (i, child) | {
                    html!{ <RowComponent key={i} ..child.clone() /> }
                }).collect::<Html>()
            }
            </div>
            <div id="controller">
                <button 
                    onclick={check_onclick} 
                    >{"check"}</button>
                <button
                    onclick={solve_onclick}
                    >{"solve"}</button>
                <button 
                    onclick={next_onclick}
                    >{"next"}</button>
            </div>
        </>
    }
}


pub fn parse_level(level_data: String) -> Vec<RowComponentProps> {
    let level_lines = level_data.lines().collect::<Vec<_>>();
    level_lines.iter().enumerate().map( | (row, line) | {
        RowComponentProps {
            row_count: row,
            children: line.clone().chars().enumerate().map(| (column, char) | {
                CellComponentProps { 
                    coordinate: (row, column),
                    value: get_index(char) 
                }
            }).collect()
        }
    } ).collect()
}

pub fn get_index(cell_symbol: char) -> usize {
    match cell_symbol {
        ' ' => 0,
        '╹' | '╺' | '╻' | '╸' => 1,
        '┃' | '━' => 2,
        '┗' | '┏' | '┛' | '┓' => 3,
        '┣' | '┻' | '┫' | '┳' => 4,
        '╋' => 5,
        _ => 0,
        // ' ' => 0,
        // '-' => 1,
        // 'I' => 2,
        // 'L' => 3,
        // 'T' => 4,
        // '+' => 5,
        // _ => 0,
    }
}