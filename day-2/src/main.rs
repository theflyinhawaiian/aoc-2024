use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result = contents.lines().map(|line| {
        line.split_whitespace().filter_map(|n| n.parse::<i64>().ok())
    }).map(|reports| {
        reports.fold((0, 0, true), |(last, asc, is_safe), curr| {
            if !is_safe { return (curr, asc, false) }
            if last == 0 { return (curr, 0, true) }
            let diff = curr - last;
            if asc == 0 { 
                if diff >= 1 && diff <= 3 { return (curr, 1, true) };
                if diff <= -1 && diff >= -3 { return (curr, -1, true) };
                return (curr, 0, false)
            } 
            else if asc == -1 && diff <= -1 && diff >= -3 { return (curr, -1, true) }
            else if asc == 1 && diff >= 1 && diff <= 3 { return (curr, 1, true) };
            return (curr, asc, false)
        })
    }).filter(|&(_, _, is_safe)| {
        is_safe
    }).count();

    println!("{}", result);
}
