use crate::{enumset, tile};

use super::{
    grid::Grid,
    finite::Finite,
    tile::{
        Square,
        Square::{Down, Left, Right, Up},
        Tile,
        
    },
};

//
fn tile_to_literals(tile: Tile<Square>, num: i32) -> String{

    match tile.enum_to_index(){
        0 => format!("-{} 0\n-{} 0\n-{} 0\n-{} 0\n",num*4+1,num*4+2,num*4+3,num*4+4),
        1 => format!("-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n{} {} {} {} 0\n-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+2,num*4+1,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+2,num*4+3,num*4+4,num*4+2,num*4+3,num*4+4,num*4+3,num*4+2,num*4+4),
        2 => format!("-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n{} {} {} {} 0\n-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+2,num*4+1,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+2,num*4+3,num*4+4,num*4+2,num*4+3,num*4+4,num*4+3,num*4+2,num*4+4),
        3 => format!("{} {} 0\n-{} -{} 0\n{} {} 0\n-{} -{} 0\n",num*4+4,num*4+2,num*4+2,num*4+4,num*4+3,num*4+1,num*4+1,num*4+3),
        4 => format!("-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n{} {} {} {} 0\n-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+2,num*4+1,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+2,num*4+3,num*4+4,num*4+2,num*4+3,num*4+4,num*4+3,num*4+2,num*4+4),
        5 => format!("-{} {} 0\n-{} {} 0\n-{} {} 0\n-{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} 0\n-{} -{} 0\n{} {} 0\n{} {} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+3,num*4+1,num*4+1,num*4+3,num*4+4,num*4+2,num*4+2,num*4+4,num*4+2,num*4+1,num*4+4,num*4+1,num*4+1,num*4+2,num*4+3,num*4+2,num*4+2,num*4+3,num*4+4,num*4+3,num*4+1,num*4+4,num*4+3,num*4+4),
        6 => format!("{} {} 0\n-{} -{} 0\n{} {} 0\n-{} -{} 0\n",num*4+4,num*4+2,num*4+2,num*4+4,num*4+3,num*4+1,num*4+1,num*4+3),
        7 => format!("{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} -{} -{} 0\n{} {} 0\n",num*4+2,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+3,num*4+2,num*4+4,num*4+2,num*4+3,num*4+4,num*4+1,num*4+2,num*4+4,num*4+3),
        8 => format!("-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n{} {} {} {} 0\n-{} -{} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+2,num*4+1,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+2,num*4+3,num*4+4,num*4+2,num*4+3,num*4+4,num*4+3,num*4+2,num*4+4),
        9 => format!("{} {} 0\n-{} -{} 0\n{} {} 0\n-{} -{} 0\n",num*4+4,num*4+2,num*4+2,num*4+4,num*4+3,num*4+1,num*4+1,num*4+3),
        10=> format!("-{} {} 0\n-{} {} 0\n-{} {} 0\n-{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} 0\n-{} -{} 0\n{} {} 0\n{} {} 0\n-{} -{} 0\n-{} -{} 0\n",num*4+3,num*4+1,num*4+1,num*4+3,num*4+4,num*4+2,num*4+2,num*4+4,num*4+2,num*4+1,num*4+4,num*4+1,num*4+1,num*4+2,num*4+3,num*4+2,num*4+2,num*4+3,num*4+4,num*4+3,num*4+1,num*4+4,num*4+3,num*4+4),
        11 => format!("{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} -{} -{} 0\n{} {} 0\n",num*4+2,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+3,num*4+2,num*4+4,num*4+2,num*4+3,num*4+4,num*4+1,num*4+2,num*4+4,num*4+3),
        12 => format!("{} {} 0\n-{} -{} 0\n{} {} 0\n-{} -{} 0\n",num*4+4,num*4+2,num*4+2,num*4+4,num*4+3,num*4+1,num*4+1,num*4+3),
        13 => format!("{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} -{} -{} 0\n{} {} 0\n",num*4+2,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+3,num*4+2,num*4+4,num*4+2,num*4+3,num*4+4,num*4+1,num*4+2,num*4+4,num*4+3),
        14 => format!("{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n{} {} 0\n-{} -{} -{} -{} 0\n{} {} 0\n",num*4+2,num*4+1,num*4+3,num*4+1,num*4+4,num*4+1,num*4+3,num*4+2,num*4+4,num*4+2,num*4+3,num*4+4,num*4+1,num*4+2,num*4+4,num*4+3),
        15 => format!("{} 0\n{} 0\n{} 0\n{} 0\n",num*4+1,num*4+2,num*4+3,num*4+4),

        _ => "".to_string()
    }
}

pub fn level_to_cnf(level: &Grid<Tile<Square>>) -> Result<String, String>{
    let mut num = 0;
    let mut knf = String::from("");
    for l in level.clone().into_iter() {
        let char_knf = tile_to_literals(l,num);
        num+=1;

        knf.push_str(&char_knf);
    }

    let columns = level.columns();
    let rows = level.rows();

    for x in 0..columns {
        for y in 0..rows {
            if y == 0 {
                knf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+1}))
            }
            if y == rows -1 {
                knf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+3}))
            }

            if x == 0 {
                knf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+4}))
            }

            if x == columns-1 {
                knf.push_str(&format!("-{} 0\n",(x+columns*y)*4+2))
            }

            if x < columns-1 {
                knf.push_str(&format!("{} -{} 0\n-{} {} 0\n", {(x+columns*y)*4+2} ,{((x+1)+columns*y)*4+4}, {(x+columns*y)*4+2}, {((x+1)+columns*y)*4+4}))
            }

            if y < rows-1 {
                knf.push_str(&format!("{} -{} 0\n-{} {} 0\n", (x+columns*y)*4+3, (x+columns*(y+1))*4+1, (x+columns*y)*4+3, (x+columns*(y+1))*4+1))
            }
        }
    }
    

    let header = format!("p cnf {} {} \n",rows*columns*4,knf.matches(" 0\n").count());
    let combine = header + &knf;

    log::info!("{}",combine);
    Ok(combine)
}

pub fn solved_to_tiles(solved: &str) -> Result<Vec<Tile<Square>>, String>{
    let mut literals = vec![];
    let mut literal = String::from("");
    for c in solved.chars() {
        // if c == '0' {
        //     break;
        // }
        if c == ' ' {

            let lit = literal.parse::<i32>();
            match lit {
                Ok(i) => {literals.push(i)},
                Err(_e) => return Ok(vec![]),
            }
            literal = String::from("");
            continue;
        }
        else{
            literal.push(c)
        }
    }
    if literal != "" {
        let last = literal.parse::<i32>();
        match last {
            Ok(i) => {literals.push(i)},
            Err(_e) => return Ok(vec![]),
        }
    }

    literals.sort_by(|a,b| a.abs().cmp(&b.abs()));

    log::info!("literals: {}",literals.len());

    let mut tiles = vec![];

    for i in 0..literals.len()/4 {
        if literals[i*4] > 0 {
            if literals[i*4+1] > 0 {
                if literals[i*4+2] > 0 {
                    if literals[i*4+3] > 0 {
                        tiles.push(Tile::ALL_CONNECTIONS)
                    } else {
                        tiles.push(tile!(Up, Right, Down))
                    }
                } else {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Up,Right, Left))
                    } else {
                        tiles.push(tile!(Up, Right))
                    }
                }
            } else {
                if literals[i*4+2] > 0 {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Up,Down,Left))
                    } else {
                        tiles.push(tile!(Up, Down))
                    }
                } else {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Up, Left))
                    } else {
                        tiles.push(tile!(Up))
                    }
                }
            }
        } else {
            if literals[i*4+1] > 0 {
                if literals[i*4+2] > 0 {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Right, Down, Left))
                    } else {
                        tiles.push(tile!(Right, Down))
                    }
                } else {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Right, Left))
                    } else {
                        tiles.push(tile!(Right))
                    }
                }
            } else {
                if literals[i*4+2] > 0 {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Down,Left))
                    } else {
                        tiles.push(tile!(Down))
                    }
                } else {
                    if literals[i*4+3] > 0 {
                        tiles.push(tile!(Left))
                    } else {
                        tiles.push(Tile::NO_CONNECTIONS)
                    }
                }
            }
        }
    }
    log::info!("tiles: {}",tiles.len());
    Ok(tiles)
}