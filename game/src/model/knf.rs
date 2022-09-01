pub fn tile_to_literals(tile: Tile<Square>,x,y) -> Result<String>{
    match tile {
        Ok(Tile(EnumSet::empty())) => Ok("-{x}{y}1 0 \n -{x}{y}2 0 \n -{x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(EnumSet::only(Up))) => Ok("{x}{y}1 0 \n -{x}{y}2 0 \n -{x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(EnumSet::only(Right))) => Ok("-{x}{y}1 0 \n {x}{y}2 0 \n -{x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(Up | Right)) => Ok("{x}{y}1 0 \n {x}{y}2 0 \n -{x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(EnumSet::only(Down))) => Ok("-{x}{y}1 0 \n -{x}{y}2 0 \n {x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(Up | Down)) => Ok("{x}{y}1 0 \n -{x}{y}2 0 \n {x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(Right | Down)) => Ok("-{x}{y}1 0 \n {x}{y}2 0 \n {x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(Up | Right | Down)) => Ok("{x}{y}1 0 \n {x}{y}2 0 \n {x}{y}3 0 \n -{x}{y}4 0 \n"),
        Ok(Tile(EnumSet::only(Left))) => Ok("-{x}{y}1 0 \n -{x}{y}2 0 \n -{x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Up | Left)) => Ok("{x}{y}1 0 \n -{x}{y}2 0 \n -{x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Right | Left)) => Ok("-{x}{y}1 0 \n {x}{y}2 0 \n -{x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Up | Right | Left)) => Ok("{x}{y}1 0 \n {x}{y}2 0 \n -{x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Down | Left)) => Ok("-{x}{y}1 0 \n -{x}{y}2 0 \n {x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Up | Down | Left)) => Ok("{x}{y}1 0 \n -{x}{y}2 0 \n {x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(Right | Down | Left)) => Ok("-{x}{y}1 0 \n {x}{y}2 0 \n {x}{y}3 0 \n {x}{y}4 0 \n"),
        Ok(Tile(EnumSet::all())) => Ok("{x}{y}1 0 \n {x}{y}2 0 \n {x}{y}3 0 \n {x}{y}4 0 \n"),
    }
}

char_to_knf(tile_character: char, x, y) -> Result<String>{
    match tile_character {
        ' ' => Ok("-{x}{y}1 0 \n -{x}{y}2 0 \n -{x}{y}3 0 \n -{x}{y}4 0 \n"),
        'L' => Ok("{x}{y}4 {x}{y}2 0 \n -{x}{y}2 -{x}{y}4 0 \n {x}{y}3 {x}{y}1 0 \n -{x}{y}1 -{x}{y}3 0 \n"),
        'T' => Ok("{x}{y}2 {x}{y}1 0 \n {x}{y}3 {x}{y}1 0 \n {x}{y}4 {x}{y}1 0 \n {x}{y}3 {x}{y}2 0 \n {x}{y}4 {x}{y}2 0 \n -{x}{y}3 -{x}{y}4 -{x}{y}1 -{x}{y}2 0 \n {x}{y}4 {x}{y}3 0 \n"),
        'I' => Ok("-{x}{y}3 {x}{y}1 0 \n -{x}{y}1 {x}{y}3 0 \n -{x}{y}4 {x}{y}2 0 \n -{x}{y}2 {x}{y}4 0 \n {x}{y}2 {x}{y}1 0 \n {x}{y}4 {x}{y}1 0 \n -{x}{y}1 -{x}{y}2 0 \n -{x}{y}3 -{x}{y}2 0 \n {x}{y}2 {x}{y}3 0 \n {x}{y}4 {x}{y}3 0 \n -{x}{y}1 -{x}{y}4 0 \n -{x}{y}3 -{x}{y}4 0 \n"),
        '-' => Ok("-{x}{y}2 -{x}{y}1 0 \n -{x}{y}1 -{x}{y}3 0 \n -{x}{y}1 -{x}{y}4 0 \n {x}{y}1 {x}{y}2 {x}{y}3 {x}{y}4 0 \n -{x}{y}2 -{x}{y}3 0 \n -{x}{y}4 -{x}{y}3 0 \n -{x}{y}2 -{x}{y}4 0 \n"),
        '+' => Ok("{x}{y}1 0 \n {x}{y}2 0 \n {x}{y}3 0 \n {x}{y}4 0 \n"),
        c => Err(format!("parsing error: unknown character {c}")),
    }
}

level_to_knf(level: &str) -> Result<String>{
    let x = 0;
    let y = 0;
    let knf = "";
    for l in level {
        if l=="\n" {
            y+=1;
            continue;
        }
        let char_knf = char_to_knf(l,x,y)
        x+=1;

        knf += char_knf;
    }
    Ok(knf)
}