use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Should have been able to read the file");

    let mut parts = contents.split(" ");

    let mut rules : HashMap<usize, Vec<usize>> = HashMap::new();
    parts.next().expect("").split_whitespace().map(|x| { 
        let mut nums = x.split("|").map(|i| i.parse::<usize>().ok()); 
        (nums.next().expect("").unwrap(), nums.next().expect("").unwrap()) 
    }).for_each(|(val, key)| {
        rules.entry(key).or_insert_with(Vec::new).push(val);
    });

    let lists: Vec<Vec<usize>> = 
        parts.next().expect("").split_whitespace()
             .map(|list| list.split(",").map(|x| x.parse::<usize>().ok().unwrap()).collect::<Vec<usize>>()).collect();

    let result : usize = lists.iter().filter_map(|list| {
        let mut valid = true;
        let mut banned_nums : Vec<usize> = Vec::new();
        for i in 0..list.len()-1 {
            if banned_nums.contains(&list[i]) {
                valid = false;
                break;
            }

            if let Some(next_bans) = rules.get(&list[i]) {
                banned_nums.extend(next_bans);
            }
        }
        if valid { Some(list[list.len()/2]) } else { None }
    }).sum();

    println!("{}", result);
}