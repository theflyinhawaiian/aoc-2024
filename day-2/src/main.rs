use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result = contents.lines().map(|line| {
        let reports: Vec<i64> = line.split_whitespace().filter_map(|n| n.parse::<i64>().ok()).collect();

        let display_value = reports.iter().fold(String::new(), |a, b| a + &b.to_string() + " ");
        let enumerated = reports.iter().enumerate().map(|(idx, _)| {
            reports.iter()
                .enumerate()
                .filter_map(move |(i, &x)| if i == idx { None } else { Some(x) })
        }).filter(|x| {
            is_safe(x.clone())
        }).count();
        println!("{}, {}", display_value, enumerated);
        enumerated >= 1
    }).filter(|&x| x)
    .count();

    // let result = contents.lines().map(|line| {
    //     let reports = line.split_whitespace().filter_map(|n| n.parse::<i64>().ok());
    //     is_safe(reports)
    // }).filter(|&x| x)
    // .count();

    println!("{}", result);
}

fn is_safe<I>(reports: I) -> bool
where I: Iterator<Item = i64> {
    let collected : Vec<i64> = reports.collect();
    let stringified = collected.iter().fold(String::new(), |a, b| a + &b.to_string() + " ");
    let result = collected.iter().fold((0, 0, true), |(last, asc, is_safe), &curr| {
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
    }).2;

    result
}