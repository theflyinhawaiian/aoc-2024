use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let lines : Vec<Vec<char>> = contents.lines().map(|line| { let vec_chars : Vec<char> = line.chars().collect(); vec_chars }).collect();

    let pyramidized : Vec<char> = (0..lines.len()).map(|row| {
        println!("{}", lines[row][0]);
        lines[row][0]
    }).collect();
    // (0..2*lines.len()-1).map(|row| { 
    //     let length = lines.len();
    //     let start_x = if row / length == 0 { 0 } else { row % length };
    //     let start_y = if row / length == 0 { row % length } else { length-1 };
    //     println!("{}, startX: {}, startY: {}, {}", row, start_x, start_y, lines[start_x][start_y]);
    //     lines[start_x][start_y]
    // }).collect();

}