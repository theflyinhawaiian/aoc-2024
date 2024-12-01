use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    for (_, val) in lines.enumerate() {
        let mut line = val.split_whitespace();
        list1.push(line.next().unwrap().parse::<u64>().unwrap());
        list2.push(line.next().unwrap().parse::<u64>().unwrap());
    }

    for val in list1 {
        let out_str = val.to_string();
        println!("{}", out_str);
    }

    for val in list2 {
        let out_str = val.to_string();
        println!("{}", out_str);
    }
}