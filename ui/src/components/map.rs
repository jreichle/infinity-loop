use game::model::coordinate::Coordinate;
use game::model::grid::Grid;
use game::model::tile::{Tile, Square};

use yew::prelude::*;
use yew::{html, Html, Properties, Callback, use_state};

#[derive(Properties, PartialEq, Clone)]
pub struct CellComponentProps {
    // coordinate: (usize, usize),
    // value: usize,
    row_number: isize,
    column_number: isize,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellComponentProps) -> Html {
    let mut map_context = use_context::<MapLayout>().expect("no ctx found");
    let (row, column) = (props.row_number.clone(), props.column_number.clone());
    let index = Coordinate { row, column };
    let cell_tile = map_context.map_grid.get(index.clone()).unwrap();
    let cell_img = get_index(cell_tile.get_value().clone());

    let angle = use_state(|| 0_usize);

    let img_path = vec![
        "data/tiles/0.svg",
        "data/tiles/1.svg",
        "data/tiles/2.svg",
        "data/tiles/3.svg",
        "data/tiles/4.svg",
        "data/tiles/5.svg",
    ];

    // let angle = use_state(|| 0_usize);
    let onclick: Callback<MouseEvent> = {
        // map_context.map_grid.get_mut(index).unwrap().rotated_clockwise(1);
        log::info!("Tile with coordinate ({}, {}) has been clicked.", row, column);
        let angle = angle.clone();
        Callback::from(move |_| angle.set((*angle + 90) % 360))
    };

    html! {
        <div class="cell">
            <img src={ img_path[cell_img] }
                onclick={onclick}
                style={format!("{}{}{}","transform:rotate(", *angle, "deg);")}
            />
        </div>
    }

}

#[derive(Properties, PartialEq, Clone)]
pub struct RowComponentProps {
    row_number: isize,
}

#[function_component(RowComponent)]
pub fn row_component(props: &RowComponentProps) -> Html {
    let map_context = use_context::<MapLayout>().expect("no ctx found");
    let (_, width) = map_context.map_grid.dimensions().to_tuple();

    html! {
        <div class="cell-row">
            {
                (0..width).into_iter().map(| column_number | {
                    html!{ 
                        <CellComponent key={column_number} row_number={props.row_number.clone()} column_number={column_number as isize} /> 
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct MapLayout {
    pub level_number: usize,
    pub map_grid: Grid<Tile<Square>>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MapComponentProps {
    pub map_layout: MapLayout,
}

#[function_component(MapComponent)]
pub fn map_component(props: &MapComponentProps) -> Html {
    // let level = parse_level(props.level_data.clone());
    
    let map_context = use_state(|| props.map_layout.clone());
    let map_grid = props.map_layout.map_grid.clone();
    let (height, _) = map_grid.dimensions().to_tuple();

    let check_onclick: Callback<MouseEvent> = {
        Callback::from(move |_| {
            log::info!("LEVEL\n{}", map_grid.to_string());
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
            <ContextProvider<MapLayout> context={(*map_context).clone()}>
            {
                (0..height).into_iter().map(| row_number | {
                    html!{ 
                        <RowComponent key={row_number} row_number={row_number as isize} /> 
                    }
                }).collect::<Html>()
            }
            </ContextProvider<MapLayout>>
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


// pub fn parse_level(level_data: String) -> Vec<RowComponentProps> {
//     let level_lines = level_data.lines().collect::<Vec<_>>();
//     level_lines.iter().enumerate().map( | (row, line) | {
//         RowComponentProps {
//             row_count: row,
//             children: line.clone().chars().enumerate().map(| (column, char) | {
//                 CellComponentProps { 
//                     coordinate: (row, column),
//                     value: get_index(char) 
//                 }
//             }).collect()
//         }
//     } ).collect()
// }

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