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

// this function takes a square Tile and decodes it depending on its position UNICODE_TILES
// e.g. for the char '┣' in position 7 in the array it is decoded into the cnf formula of tiles with 3 open ends
// this formula originates in the 4 positions the tile can have where all but one of the sides have open ends
// let's say the sides are A, B, C and D:
// A ∧ B ∧ C ∧ ¬D ∨ ¬A ∧ B ∧ C ∧ D ∨ A ∧ ¬B ∧ C ∧ D ∨ A ∧ B ∧ ¬C ∧ D
// becomes
// (B ∨ A) ∧ (C ∨ A) ∧ (D ∨ A) ∧ (C ∨ B) ∧ (D ∨ B) ∧ (¬C ∨ ¬D ∨ ¬A ∨ ¬B) ∧ (D ∨ C)
// since there are many tiles the sides are given numbers, beginning in the top left with 1-4, then moving to the right 5-8 and so forth
// to achieve this the number of each tile is given to the function and multiplied by 4 before adding 1 to 4 to it. The most up left tile is numbered 0
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

// this function runs tile_to_literal for every tile in the grid
// this also sets all literals that are at the edge of the puzzle to false
// this also assures that adjacent sides of tiles have the same value, so they either connect or they don't but not that one tile has a connection and one doesn't

pub fn level_to_cnf(level: &Grid<Tile<Square>>) -> Result<String, String>{
    let mut num = 0;
    let mut cnf = String::from("");
    //run tile_to_literal for all tiles
    for l in level.clone().into_iter() {
        let char_cnf = tile_to_literals(l,num);
        num+=1;

        cnf.push_str(&char_cnf);
    }

    let columns = level.columns();
    let rows = level.rows();

    for x in 0..columns {
        for y in 0..rows {
            //set literals at the edge of the puzzle false
            if y == 0 {
                cnf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+1}))
            }

            if y == rows -1 {
                cnf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+3}))
            }

            if x == 0 {
                cnf.push_str(&format!("-{} 0\n",{(x+columns*y)*4+4}))
            }

            if x == columns-1 {
                cnf.push_str(&format!("-{} 0\n",(x+columns*y)*4+2))
            }

            //assure that adjacent tiles have the same value
            if x < columns-1 {
                cnf.push_str(&format!("{} -{} 0\n-{} {} 0\n", {(x+columns*y)*4+2} ,{((x+1)+columns*y)*4+4}, {(x+columns*y)*4+2}, {((x+1)+columns*y)*4+4}))
            }

            if y < rows-1 {
                cnf.push_str(&format!("{} -{} 0\n-{} {} 0\n", (x+columns*y)*4+3, (x+columns*(y+1))*4+1, (x+columns*y)*4+3, (x+columns*(y+1))*4+1))
            }
        }
    }
    
    //add header for the cnf file
    let header = format!("p cnf {} {} \n",rows*columns*4,cnf.matches(" 0\n").count());
    let combine = header + &cnf;

    log::info!("{}",combine);
    Ok(combine)
}

// this function takes a string of signed literals and creates the corresponding tiles
// first it puts all literals into an array and sorts them by their absolute values
// then it runs through that array by increments of 4 and depending on which literals are set to true it creates tiles with connections on these sides
pub fn solved_to_tiles(solved: &str) -> Result<Vec<Tile<Square>>, String>{
    let mut literals = vec![];
    let mut literal = String::from("");
    for c in solved.chars() {
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

    //run through the literals in increments of 4, creating the tiles depending on those

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