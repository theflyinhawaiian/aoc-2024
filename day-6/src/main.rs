use std::fs;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let chars : Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();

    let starting_pos = cols(cols(cols(chars)));

    let (x, y) = get_starting_pos(&starting_pos);

    println!("{}, {}", x, y);
    
    print(&starting_pos);
}

fn print(lines: &Vec<Vec<char>>){
    println!("{}", lines.clone().iter().map(|x| { x.iter().join(" ") }).join("\n"));
}

fn cols(lines: Vec<Vec<char>>) -> Vec<Vec<char>>{
    (0..lines.len()).map(|row| {
        (0..lines[lines.len()-1-row].len()).map(|col| {
            lines[col][lines.len()-1-row]
        }).collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>()
}

fn get_starting_pos(lines: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..lines.len(){
        for j in 0..lines[i].len(){
            if lines[i][j] == '^' {
                return (i, j)
            }
        }
    }

    (lines.len(), lines.len())
}