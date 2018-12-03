use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn solve() {
    println!("Solving day 1:");
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
        let data_file = File::open("inputs/day01.input").unwrap();  // TODO: Move out of loop
        let data = BufReader::new(&data_file);  // TODO: Move out of loop
        for line in data.lines() {  // Maybe change into a while let?
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(), Ok(505));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(), Ok(72330))
    }


}