use super::{grid::Grid, tile::Tile};

/// char to tile mapping
///
/// | Connections | Character | Unicode            |
/// |:------------|:----------|:-------------------|
/// | 0           | ' '       | ' '                |
/// | 1           | '-'       | '╹', '╺', '╻', '╸' |
/// | 2           | 'I'       | '┃', '━'           |
/// | 2           | 'L'       | '┗', '┏', '┛', '┓' |
/// | 3           | 'T'       | '┣', '┻', '┫', '┳' |
/// | 4           | '+'       | '╋'                |
pub fn char_to_tile(tile_character: char) -> Result<Tile, String> {
    match tile_character {
        ' ' => Ok(Tile::new(0b0000)),
        '-' => Ok(Tile::new(0b1000)),
        'I' => Ok(Tile::new(0b1010)),
        'L' => Ok(Tile::new(0b1100)),
        'T' => Ok(Tile::new(0b1110)),
        '+' => Ok(Tile::new(0b1111)),
        c => Err(format!("parsing error: unknown character {c}")),
    }
}

pub fn unicode_to_tile(tile_character: char) -> Result<Tile, String> {
    match tile_character {
        ' ' => Ok(Tile::new(0b0000)),
        '╹' => Ok(Tile::new(0b1000)),
        '╺' => Ok(Tile::new(0b0100)),
        '┗' => Ok(Tile::new(0b1100)),
        '╻' => Ok(Tile::new(0b0010)),
        '┃' => Ok(Tile::new(0b1010)),
        '┏' => Ok(Tile::new(0b0110)),
        '┣' => Ok(Tile::new(0b1110)),
        '╸' => Ok(Tile::new(0b0001)),
        '┛' => Ok(Tile::new(0b1001)),
        '━' => Ok(Tile::new(0b0101)),
        '┻' => Ok(Tile::new(0b1101)),
        '┓' => Ok(Tile::new(0b0011)),
        '┫' => Ok(Tile::new(0b1011)),
        '┳' => Ok(Tile::new(0b0111)),
        '╋' => Ok(Tile::new(0b1111)),
        c => Err(format!("parsing error: unknown character {c}")),
    }
}

/// parses level from string representation
///
/// expects newline delimited string
/// relies on internal vector layout in grid
pub fn parse_level<F>(leveldata: &str, converter: F) -> Result<Grid<Tile>, String>
where
    F: Fn(char) -> Result<Tile, String>,
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
pub fn serialize_level<F: Fn(Tile) -> char>(grid: Grid<Tile>, converter: F) -> String {
    grid.elements2()
    .into_iter()
    .map(converter)
    .collect::<Vec<char>>()
    .chunks(grid.columns())
    .map(|chunk| chunk.into_iter().collect())
    .collect::<Vec<String>>()
    .join("\n")
}

pub const LEVEL_MALFORMED: &str = " ";

/// first 20 levels of android game infinity loop
pub const TEST_LEVELS: [&str; 20] = [
    "LTL\nLTL",
    "LLLL\nLLLL",
    "LTL\nT+T\nLTL",
    "- -\nI I\n- -",
    "LITL\nTTTI\nITTT\nLTIL",
    " LL-\nL++L\n-II \n -- ",
    "LIIL\nLLLL\n-II-\n----",
    "LIIIL\n--T--\n-I+I-\n--T--\nLIIIL",
    "LTTIL\nIITIT\nTT+TT\nTITII\nLITTL",
    " LLLL \nLLLLLL\nLLLLLL\n LLLL ",
    " LL \n-TT-\nLTTL\nLTTL\nLIIL",
    "- --\nTTLI\nL+IL\n-TL-\nIL+T\nLITL",
    "-T-\n-T-\n-I-\n-I-\n-T-\n-T-",
    "-TL\n-TL\nLL \nIT-\n-LL\n  -",
    "-TL\nLTI\nILT\nI-L\nLI-",
    "-LL-\nL+L-\n-T- \nLL -\nTL -\n-   ",
    "--LL\nL+LI\nLT -\n-TL-\n-LTL\nLTT-\n -L-",
    "--L\n-LL\nL+-\nTTL\nLIL\n-I-",
    "---\nITL\nII \nIT-\nIL-\nLI-",
    "L- \nTL-\nIII\nLTT\nLTL\n-LL\n -L",
];

/*
pub const LEVEL_1: &str = "
LTL
LTL";

const LEVEL_2: &str = "
LLLL
LLLL";

const LEVEL_3: &str = "
LTL
T+T
LTL";

const LEVEL_4: &str = "
- -
I I
- -";

const LEVEL_5: &str = "
LITL
TTTI
ITTT
LTIL";

const LEVEL_6: &str = "
 LL-
L++L
-II
 -- ";

const LEVEL_7: &str = "
LIIL
LLLL
-II-
----";

const LEVEL_8: &str = "
LIIIL
--T--
-I+I-
--T--
LIIIL";

const LEVEL_9: &str = "
LTTIL
IITIT
TT+TT
TITII
LITTL";

const LEVEL_10: &str = "
 LLLL
LLLLLL
LLLLLL
 LLLL ";

const LEVEL_11: &str = "
 LL
-TT-
LTTL
LTTL
LIIL";

const LEVEL_12: &str = "
- --
TTLI
L+IL
-TL-
IL+T
LITL";

const LEVEL_13: &str = "
-T-
-T-
-I-
-I-
-T-
-T-";

const LEVEL_14: &str = "
-TL
-TL
LL
IT-
-LL
  -";

const LEVEL_15: &str = "
-TL
LTI
ILT
I-L
LI-";

const LEVEL_16: &str = "
-LL-
L+L-
-T-
LL -
TL -
-   ";

const LEVEL_17: &str = "
--LL
L+LI
LT -
-TL-
-LTL
LTT-
 -L-";

const LEVEL_18: &str = "
--L
-LL
L+-
TTL
LIL
-I-";

const LEVEL_19: &str = "
---
ITL
II
IT-
IL-
LI-";

const LEVEL_20: &str = "
L-
TL-
III
LTT
LTL
-LL
 -L";
*/
