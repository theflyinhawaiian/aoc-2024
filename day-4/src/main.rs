use std::fs;

use regex::Regex;

fn main(){
    p2();
}

fn p2() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let lines : Vec<Vec<char>> = 
        contents.lines()
                .map(|line| { 
                    let vec_chars : Vec<char> = 
                        line.chars().map(|x| x).collect(); 
                    vec_chars
                }).collect();

    let mut count = 0;
    for i in 1..lines.len()-1 {
        for j in 1..lines[i].len()-1{
            if lines[i][j] == 'A' {

                let left_diagonal = get_left_diagonal( &lines, i, j);
                let right_diagonal = get_right_diagonal(&lines, i, j);
                
                if is_mas(left_diagonal) && is_mas(right_diagonal) { count += 1; }
            }
        }
    }

    println!("{}", count);
}

fn get_left_diagonal(arr: &Vec<Vec<char>>, i: usize, j: usize) -> String {
    let mut diagonal_1 = Vec::new();
    diagonal_1.push(arr[i-1][j-1]);
    diagonal_1.push(arr[i][j]);
    diagonal_1.push(arr[i+1][j+1]);
    diagonal_1.iter().map(|x| x).collect::<String>()
}

fn get_right_diagonal(arr: &Vec<Vec<char>>, i: usize, j: usize) -> String {
    let mut diagonal_2 = Vec::new();
    diagonal_2.push(arr[i-1][j+1]);
    diagonal_2.push(arr[i][j]);
    diagonal_2.push(arr[i+1][j-1]);
    diagonal_2.iter().map(|x| x).collect::<String>()
}

fn is_mas(word: String) -> bool {
    word == "SAM" || word == "MAS"
}

fn p1() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let lines : Vec<Vec<char>> = 
        contents.lines()
                .map(|line| { 
                    let vec_chars : Vec<char> = 
                        line.chars().map(|x| x).collect(); 
                    vec_chars
                }).collect();

    let rows : Vec<String> =
        lines.iter().map(|x| x.into_iter().collect::<String>()).collect();


    let cols : Vec<String> =
        (0..lines.len()).map(|row| {
            (0..lines[row].len()).map(|col| {
                lines[col][row]
            }).collect::<String>()
        }).collect();

    let rotated_cw : Vec<String> = 
    (0..2*lines.len()-1).map(|row| { 
        let length = lines.len();
        let boundary_idx = row % length;
        let start_x = if row / length == 0 { 0 } else { boundary_idx + 1 };
        let start_y = if row / length == 0 { boundary_idx } else { length - 1 };
        let diagonal_length = if row / length == 1 { (length - 1) - (row % length) } else { row + 1 };
        (0..diagonal_length).map(|offset| {
            lines[start_x + offset][start_y - offset]
        }).collect()
    }).collect();

    let rotated_ccw : Vec<String> =
    (0..2*lines.len()-1).map(|row| { 
        let length = lines.len();
        let boundary_idx = row % length;
        let start_x = if row / length == 0 { 0 } else { boundary_idx + 1 };
        let start_y = if row / length == 0 { (length - 1) - (row % length) } else { 0 };
        let diagonal_length = if row / length == 1 { (length - 1) - (row % length) } else { row + 1 };
        (0..diagonal_length).map(|offset| {
            lines[start_x + offset][start_y + offset]
        }).collect()
    }).collect();

    let rev_rows = rev_lines(&rows);
    let rev_cols = rev_lines(&cols);
    let rev_rotated_cw = rev_lines(&rotated_cw);
    let rev_rotated_ccw = rev_lines(&rotated_ccw);

    let row_matches = get_matches(&rows);
    let col_matches = get_matches(&cols);
    let rotated_cw_matches = get_matches(&rotated_cw);
    let rotated_ccw_matches = get_matches(&rotated_ccw);
    let rev_row_matches = get_matches(&rev_rows);
    let rev_col_matches = get_matches(&rev_cols);
    let rev_rotated_cw_matches = get_matches(&rev_rotated_cw);
    let rev_rotated_ccw_matches = get_matches(&rev_rotated_ccw);

    println!("rotated_cw matches: {}", rotated_cw_matches);
    println!("rotated_ccw matches: {}", rotated_ccw_matches);
    println!("row matches: {}", row_matches);
    println!("col matches: {}", col_matches);
    println!("rev row matches: {}", rev_row_matches);
    println!("rev col matches: {}", rev_col_matches);
    println!("rev rotated_cw matches: {}", rev_rotated_cw_matches);
    println!("rev rotated_ccw matches: {}", rev_rotated_ccw_matches);
    println!("total: {}", row_matches + col_matches + rotated_cw_matches + rotated_ccw_matches + rev_row_matches + rev_col_matches + rev_rotated_cw_matches + rev_rotated_ccw_matches);
    
}

fn rev_lines(lines: &Vec<String>) -> Vec<String> {
    lines.iter().map(|x| x.chars().rev().collect::<String>()).collect::<Vec<String>>()
}

fn get_matches(lines: &Vec<String>) -> usize {
    lines.iter().fold(0, |acc, line| {
        acc + num_xmases(&line)
    })
}

fn num_xmases(line: &String) -> usize {
    let regex = Regex::new(r"(XMAS)").unwrap();
    regex.captures_iter(&line).count()
}