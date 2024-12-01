use std::collections::HashMap;
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

    // Create a frequency map of elements in list2
    let mut frequency_map = HashMap::new();
    for &item in &list2 {
        *frequency_map.entry(item).or_insert(0) += 1;
    }

    let mut similarity_score = 0;

    // Count how many times each element of list1 appears in list2 using the frequency map
    for &item in &list1 {
        let count = frequency_map.get(&item).unwrap_or(&0); // Get the count, default to 0 if not found
        similarity_score += item * count;
    }

    println!("{}", similarity_score);
}