pub fn verse(n: i32) -> String {
    return match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{0:} bottles of beer on the wall, {0:} bottles of beer.\nTake one down and pass it around, {1:} bottles of beer on the wall.\n", n, n-1),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = vec![];
    for n in (end..start+1).rev() {
        song.push(verse(n));
    }
    song.join("\n")
}
