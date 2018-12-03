use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn solve() {
    println!("Solving day two:");
    if let Ok(val) = solve_part1() {
        println!("The checksum of lists of box ids: {}", val);
    }
    if let Ok(val) = solve_part2() {
        println!("The correct box id: {}", val);
    }
}


fn solve_part1() -> Result<i64, &'static str> {
    let data_file = File::open("inputs/day02.input").unwrap();
    let data_file = BufReader::new(data_file);  // Shadowing!
    let mut count_twos = 0;
    let mut count_threes = 0;
    for line in data_file.lines() {
        let (twos, threes) = twos_and_threes(&line.unwrap());
        count_twos += twos as i64;
        count_threes += threes as i64;
    }
    Ok(count_twos * count_threes)
}


fn solve_part2() -> Result<String, &'static str> {
    let df1 = File::open("inputs/day02.input").unwrap();
    let df1 = BufReader::new(df1);

    for line1 in df1.lines().filter_map(|result| result.ok()) {
        let df2 = File::open("inputs/day02.input").unwrap();
        let mut df2 = BufReader::new(&df2);
        for line2 in df2.lines().filter_map(|result| result.ok()) {
            if let Ok((diff, common)) = compare_ids(&line1, &line2) {
                if diff == 1 {
                    return Ok(common);
                }
            } 
        }
    }
    Err("Did not find an id!")
}


fn twos_and_threes(input_string: &str) -> (bool, bool) {
    let mut characters: HashMap<char, i32> = HashMap::new();
    for character in input_string.chars() {
        if characters.contains_key(&character) {
            *characters.get_mut(&character).unwrap() += 1;
        } else {
            characters.insert(character, 1);
        }
    }

    let mut two: bool = false;
    let mut three: bool = false;
    
    for (_, val) in characters.into_iter() {
        if val == 2 {
            two = true;
        } else if val == 3 {
            three = true;
        }
    }

    (two, three)    
}

fn compare_ids(id1: &String, id2: &String) -> Result<(i32, String), &'static str> {
    if id1.len() != id2.len() {
        return Err("Lengths of ids was different! Unacceptable.");
    }

    let common: String = id1.chars()
                        .zip(id2.chars())
                        .filter(|(x, y)| x == y)
                        .map(|(x, _)| x as char)
                        .collect();

    let diff = id1.len() - common.len();
    Ok((diff as i32, common))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(), Ok(5658));
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(), Ok("nmgyjkpruszlbaqwficavxneo".to_string()));
    }
}