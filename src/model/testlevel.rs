use enumset::EnumSet;

use super::{
    grid::Grid,
    tile::{
        Square::{self, Down, Left, Right, Up},
        Tile,
    },
};

/// char to tile mapping
///
/// assign a character to represent each equivalence class under rotational symmetry
///
/// | Connections | Character | Unicode                 |
/// |:------------|:----------|:------------------------|
/// | 0           | ' '       | `[ ]`                   |
/// | 1           | '-'       | `[╹]`/`[╺]`/`[╻]`/`[╸]` |
/// | 2           | 'I'       | `[┃]`/`[━]`             |
/// | 2           | 'L'       | `[┗]`/`[┏]`/`[┛]`/`[┓]` |
/// | 3           | 'T'       | `[┣]`/`[┻]`/`[┫]`/`[┳]` |
/// | 4           | '+'       | `[╋]`                   |
pub fn char_to_tile(tile_character: char) -> Result<Tile<Square>, String> {
    match tile_character {
        ' ' => Ok(Tile(EnumSet::empty())),
        '-' => Ok(Tile(!!Up)),
        'I' => Ok(Tile(Up | Down)),
        'L' => Ok(Tile(Up | Right)),
        'T' => Ok(Tile(Up | Right | Down)),
        '+' => Ok(Tile(EnumSet::all())),
        c => Err(format!("parsing error: unknown character {c}")),
    }
}

pub fn unicode_to_tile(tile_character: char) -> Result<Tile<Square>, String> {
    match tile_character {
        ' ' => Ok(Tile(EnumSet::empty())),
        '╹' => Ok(Tile(!!Up)),
        '╺' => Ok(Tile(!!Right)),
        '┗' => Ok(Tile(Up | Right)),
        '╻' => Ok(Tile(!!Down)),
        '┃' => Ok(Tile(Up | Down)),
        '┏' => Ok(Tile(Right | Down)),
        '┣' => Ok(Tile(Up | Right | Down)),
        '╸' => Ok(Tile(!!Left)),
        '┛' => Ok(Tile(Up | Left)),
        '━' => Ok(Tile(Right | Left)),
        '┻' => Ok(Tile(Up | Right | Left)),
        '┓' => Ok(Tile(Down | Left)),
        '┫' => Ok(Tile(Up | Down | Left)),
        '┳' => Ok(Tile(Right | Down | Left)),
        '╋' => Ok(Tile(EnumSet::all())),
        c => Err(format!("parsing error: unknown character {c}")),
    }
}

/// parses level from string representation
///
/// expects newline delimited string
/// relies on internal vector layout in grid
pub fn parse_level<A, F>(leveldata: &str, converter: F) -> Result<Grid<A>, String>
where
    F: Fn(char) -> Result<A, String>,
{
    let lines = leveldata.lines().collect::<Vec<_>>();

    let rows = lines.len();
    let columns = lines.get(0).map(|s| s.len()).unwrap_or(0);

    // all rows must have same length
    if lines.iter().any(|s| s.len() != columns) {
        return Err(format!("All rows must have same length: {leveldata}"));
    }

    lines
        .concat()
        .chars()
        .map(converter)
        .collect::<Result<_, _>>()
        .map(|v| Grid::new(rows, columns, v))
}

/// relies on internal vector layout in grid
pub fn serialize_level<A: Clone, F: Fn(A) -> char>(grid: Grid<A>, converter: F) -> String {
    grid.elements2()
        .into_iter()
        .map(converter)
        .collect::<Vec<char>>()
        .chunks(grid.columns())
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

pub const LEVEL_MALFORMED: &str = " ";

/// first 20 levels of android game infinity loop
pub const TEST_LEVELS: [&str; 20] = [
    /* 01 */ "LTL\nLTL",
    /* 02 */ "LLLL\nLLLL",
    /* 03 */ "LTL\nT+T\nLTL",
    /* 04 */ "- -\nI I\n- -",
    /* 05 */ "LITL\nTTTI\nITTT\nLTIL",
    /* 06 */ " LL-\nL++L\n-II \n -- ",
    /* 07 */ "LIIL\nLLLL\n-II-\n----",
    /* 08 */ "LIIIL\n--T--\n-I+I-\n--T--\nLIIIL",
    /* 09 */ "LTTIL\nIITIT\nTT+TT\nTITII\nLITTL",
    /* 10 */ " LLLL \nLLLLLL\nLLLLLL\n LLLL ",
    /* 11 */ " LL \n-TT-\nLTTL\nLTTL\nLIIL",
    /* 12 */ "- --\nTTLI\nL+IL\n-TL-\nIL+T\nLITL",
    /* 13 */ "-T-\n-T-\n-I-\n-I-\n-T-\n-T-", // requires branching
    /* 14 */ "-TL\n-TL\nLL \nIT-\n-LL\n  -",
    /* 15 */ "-TL\nLTI\nILT\nI-L\nLI-",
    /* 16 */ "-LL-\nL+L-\n-T- \nLL -\nTL -\n-   ",
    /* 17 */ "--LL\nL+LI\nLT -\n-TL-\n-LTL\nLTT-\n -L-",
    /* 18 */ "--L\n-LL\nL+-\nTTL\nLIL\n-I-",
    /* 19 */ "---\nITL\nII \nIT-\nIL-\nLI-",
    /* 20 */ "L- \nTL-\nIII\nLTT\nLTL\n-LL\n -L",
];

pub const TRIVIAL_LEVEL: &str = "-I-";

pub const MULTIPLE_SOLUTIONS: &str = "----\n----\n----\n----";

#[cfg(test)]
mod tests {

    use crate::model::{
        testlevel::unicode_to_tile,
        tile::{Square, Tile},
    };

    #[quickcheck]
    fn display_then_unicode_to_tile_is_identity(tile: Tile<Square>) -> bool {
        let tile_char = tile
            .to_string()
            .chars()
            .next()
            .expect("expected single element string");
        tile == unicode_to_tile(tile_char).unwrap()
    }
}
