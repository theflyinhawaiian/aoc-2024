use regex::Regex;
use std::fs;

fn main() {
    let regex = Regex::new(r"(do\(\)|don\'t\(\)|mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let mut enabled = true;
    let mut products : Vec<u64> = Vec::new();
    for (_, [str]) in regex.captures_iter(&contents).map(|x| x.extract()){
        if str.contains("don\'t") {
            enabled = false;
        } else if str.contains("do") {
            enabled = true;
            continue;
        }

        if enabled {
            let product = str[4..str.len()-1].split(',').filter_map(|x| x.parse::<u64>().ok()).fold(1, |acc, next| acc * next);
            products.push(product);
        }
    }

    let sum : u64 = products.iter().sum();

    println!("{}", sum.to_string());
}