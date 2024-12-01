use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    for (_, val) in lines.enumerate() {
        let mut line = val.split_whitespace();
        list1.push(line.next().unwrap().parse::<i64>().unwrap());
        list2.push(line.next().unwrap().parse::<i64>().unwrap());
    }

    list1.sort();
    list2.sort();

    let total_diff : i64 = list1
        .iter()
        .zip(list2.iter()) // Pair up elements from both lists
        .map(|(a, b)| (a - b).abs()) // Compute the difference
        .sum();

    println!("total diff: {}", total_diff);
}