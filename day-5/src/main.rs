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
        let mut banned_nums : Vec<usize> = Vec::new();
        let mut result = 0;
        for i in 0..list.len() {
            if banned_nums.contains(&list[i]) {
                // construct valid list from here
                let valid_list = cons(&list, &rules);
                println!("original list: {}\nnew list: {}", list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","), 
                valid_list.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
                result = valid_list[valid_list.len()/2];
                break;
            }

            if let Some(next_bans) = rules.get(&list[i]) {
                banned_nums.extend(next_bans);
            }
        }

        if result > 0 { Some(result) } else { None }
    }).sum();

    println!("{}", result);
}

fn cons(original_list: &Vec<usize>, rules : &HashMap<usize, Vec<usize>>) -> Vec<usize>{
    let mut copied = original_list.clone();
    (0..original_list.len()).fold(Vec::new(),|curr, _| {
        build(&curr, &mut copied, rules)
    })
}

fn build(new_list: &Vec<usize>, list : &mut Vec<usize>, rules : &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut diff = list.clone();
    diff.retain(|x| !new_list.contains(x));
    let mut updated = new_list.clone();
    for i in 0..diff.len() {
        if comes_last(diff[i], &diff, rules) {
            updated.push(diff[i]);
            return updated
        }
    }
    
    panic!("get_last really should return something")
}

fn comes_last(x : usize, list : &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    list.iter().all(|i| {
        if let Some(rule) = rules.get(i) {
            !rule.contains(&x)
        } else {
            true
        }
    })
}