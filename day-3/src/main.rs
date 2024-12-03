use regex::Regex;
use std::fs;

fn main() {
    let regex = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)").unwrap();
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let result = regex
        .captures_iter(&contents)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [next])| {
            acc + next.split(',').filter_map(|x| x.parse::<i64>().ok())
                .fold(1, |acc, num| acc * num)
        });

    println!("{}", result);
}