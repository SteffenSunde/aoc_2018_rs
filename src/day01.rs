use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::io::Seek;

pub fn solve() {
    if let Ok(val) = solve_part1() {
        println!("Total change in frequency: {}", val);
    }
    if let Ok(val) = solve_part2() {
        println!("First repeat: {}", val);
    }
}


fn solve_part1() -> Result<i64, &'static str> {
    let mut total_change: i64 = 0;
    let data_file = File::open("inputs/day01.input").unwrap();
    let data_file = BufReader::new(data_file);  // Shadowing!
    for line in data_file.lines() {
        total_change = total_change + line.unwrap().parse::<i64>().unwrap();
    }
    Ok(total_change)
}


fn solve_part2() -> Result<i64, &'static str> {
    let mut visited: HashMap<i64, bool> = HashMap::new();
    let mut first_repeat: Option<i64> = None;
    let mut total_change: i64 = 0;
    while first_repeat.is_none() {
        let data_file = File::open("inputs/day01.input").unwrap();
        let data = BufReader::new(&data_file);
        for line in data.lines() {
            total_change = total_change + line.unwrap().parse::<i64>().unwrap();
            if visited.contains_key(&total_change) {
                first_repeat = Some(total_change);
                break;
            } else {
                visited.insert(total_change, true);
            }
        }
    }
    if let Some(result) = first_repeat {
        Ok(result)
    } else {
        Err("Couldn't find a repeated frequency!")
    }
}
