use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Check neighborhood of symbol for part numbers
// TODO: Find the start and end of a part number
// TODO: When a part number is consumed, mark the locations as visited
// TODO: Add part number to sum
fn day_3_part_1(fname: &str) {
    let file = File::open(fname).expect(format!("Unable to open {}", fname).as_str());
    let lines = BufReader::new(file).lines();
    let schematic = lines
        .map(|x| match x {
            Ok(x) => x.chars().collect::<Vec<char>>(),
            Err(_) => {
                panic!("at the disco")
            }
        })
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![];
    schematic.iter().for_each(|line| {
        let mut v = vec![];
        line.iter().for_each(|_| v.push(false));
        visited.push(v);
    });
    let sum_of_parts = find_part_numbers(&schematic, &mut visited);
    println!("{}", sum_of_parts);
    let mut visited = vec![];
    for line in &schematic {
        let mut v = vec![];
        for c in line.iter() {
            v.push(false);
        }
        visited.push(v);
    }
    let sum_of_gear_ratios = find_gear_ratios(&schematic, &mut visited);
    println!("{}", sum_of_gear_ratios)
}

fn find_part_numbers(schematic: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> u32 {
    let mut part_numbers: Vec<u32> = vec![];
    for (r_idx, r) in schematic.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if is_symbol(*c) {
                let neighbors = build_neighborhood((r_idx, c_idx));
                let symbol_parts = get_parts_near_symbol(&neighbors, schematic, visited);
                symbol_parts
                    .iter()
                    .for_each(|x| part_numbers.push(x.clone()));
            }
        }
    }
    part_numbers.iter().sum::<u32>()
}
fn find_gear_ratios(schematic: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> u32 {
    let mut gear_ratios: Vec<u32> = vec![];
    for (r_idx, r) in schematic.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if is_symbol(*c) {
                let neighbors = build_neighborhood((r_idx, c_idx));
                let symbol_parts = get_gear_ratio_near_symbol(&neighbors, schematic, visited);
                symbol_parts
                    .iter()
                    .for_each(|x| gear_ratios.push(x.clone()));
            }
        }
    }
    gear_ratios.iter().sum::<u32>()
}
fn get_parts_near_symbol(
    neighbors: &HashSet<(usize, usize)>,
    schematic: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> Vec<u32> {
    neighbors
        .iter()
        .map(|x| {
            if schematic[x.0][x.1].is_numeric() && visited[x.0][x.1] != true {
                let loc = find_start_and_end((x.0, x.1), &schematic);
                visited[loc.0][loc.1 .0..loc.1 .1]
                    .iter_mut()
                    .for_each(|mut x| *x = true);
                Some(concat(&schematic[loc.0][loc.1 .0..loc.1 .1]))
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .map(|x| x.unwrap())
        .collect::<Vec<u32>>()
}
fn get_gear_ratio_near_symbol(
    neighbors: &HashSet<(usize, usize)>,
    schematic: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> Option<u32> {
    let parts = neighbors
        .iter()
        .map(|x| {
            if schematic[x.0][x.1].is_numeric() && visited[x.0][x.1] != true {
                let loc = find_start_and_end((x.0, x.1), &schematic);
                visited[loc.0][loc.1 .0..loc.1 .1]
                    .iter_mut()
                    .for_each(|mut x| *x = true);
                Some((
                    concat(&schematic[loc.0][loc.1 .0..loc.1 .1]),
                    (loc.0, (loc.1 .0, loc.1 .1)),
                ))
            } else {
                None
            }
        })
        .filter(Option::is_some)
        .map(|x| x.unwrap())
        .collect::<Vec<(u32, (usize, (usize, usize)))>>();
    parts.iter().for_each(|(_, (row, (start, end)))| {
        visited[*row][*start..*end]
            .iter_mut()
            .for_each(|mut x| *x = false)
    });
    if parts.len() == 2 {
        return Some(parts[0].0 * parts[1].0);
    }
    None
}
fn concat(vec: &[char]) -> u32 {
    vec.iter()
        .fold(0, |acc, elem| acc * 10 + elem.to_digit(10).unwrap())
}
fn is_symbol(c: char) -> bool {
    if c == '.' || c.is_numeric() {
        return false;
    }
    true
}
fn check_numeric(c: char) -> Result<(), ()> {
    match c.is_numeric() {
        true => Ok(()),
        false => Err(()),
    }
}
fn find_start_and_end(loc: (usize, usize), line: &Vec<Vec<char>>) -> (usize, (usize, usize)) {
    let line = &line[loc.0];
    let chars = line.split_at(loc.1);
    let mut rev_offset: usize = 0;
    let rev = chars.0.iter().rev().try_for_each(|x| {
        if x.is_numeric() {
            rev_offset += 1;
            return Ok(());
        }
        Err(())
    });
    let mut forward_offset = 0;
    let forward = chars.1.iter().try_for_each(|x| {
        if x.is_numeric() {
            forward_offset += 1;
            return Ok(());
        }
        Err(())
    });
    // println!(
    //     "{:?}",
    //     &line[loc.1.saturating_sub(rev_offset)..loc.1 + forward_offset]
    // );
    return (loc.0, (loc.1 - rev_offset, loc.1 + forward_offset));
}
fn build_neighborhood(loc: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut neighbors = HashSet::new();
    for r in -1..2 {
        let r_idx = loc.0.saturating_add_signed(r);
        for c in -1..=1 {
            let c_idx = loc.1.saturating_add_signed(c);
            neighbors.insert((r_idx, c_idx));
        }
    }
    // println!(
    //     "location: {}, {} - neighbors: {:?}",
    //     loc.0, loc.1, neighbors
    // );
    neighbors
}
fn check_neighborhood(loc: (usize, usize), schematic: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let neighbors = build_neighborhood(loc);
    neighbors.iter().for_each(|x| println!("{:?}", x));
    neighbors
}
// fn lines_to_vec(lines: Lines<BufReader<File>>) -> Vec<Vec<&str>> {
//
// }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_3_part_1() {
        day_3_part_1("input/day_3_input.txt")
    }
    #[test]
    fn test_day_3_part_1_example() {
        day_3_part_1("input/day_3_example.txt")
    }
}
