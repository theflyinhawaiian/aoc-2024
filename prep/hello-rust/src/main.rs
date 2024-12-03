use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    println!("{}", contents);
}