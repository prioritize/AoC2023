use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Symbol {
    r: usize,
    c: usize,
}
// TODO: Store Vector
// TODO: Create vistor vector
// TODO: Iterate Vector to locate and store symbols
// TODO: Check neighborhood of symbol for part numbers
// TODO: Find the start and end of a part number
// TODO: When a part number is consumed, mark the locations as visited
// TODO: Add part number to sum
fn day_3_part_1(fname: &str) {
    let file = File::open(fname).expect(format!("Unable to open {}", fname).as_str());
    let lines = BufReader::new(file).lines();
    // Collect all the lines of the file into a vector
    let schematic = lines.map(|x| {
        match x {
            Ok(x) => {x.chars().collect::<Vec<char>>()},
            Err(_) => {panic!("at the disco")}
        }
    }).collect::<Vec<Vec<char>>>();
    // Create a storage location for symbols
    let mut visited = vec![vec![]];
    for line in &schematic {
        let mut v = vec![];
        for c in line.iter() {
            v.push(false);
        }
        visited.push(v);
    }

    // Iterate the vectors to find all values that aren't numeric or periods
    let mut symbols = vec![];
    for (r_idx, r)  in schematic.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if is_symbol(*c) {
               check_neighborhood((r_idx, c_idx), &schematic);
            }
        }

    }
}
fn is_symbol(c: char) -> bool {
    if c == '.' || c.is_numeric() {
        return false;
    }
    true
}
fn check_numeric(c: char) -> Result<(), ()> {
   match c.is_numeric() {
       true => {Ok(())}
       false => {Err(())}
   }
}
fn find_start_and_end(idx: usize, line: String) -> (usize, usize){
    let chars = line.split_at(idx);
    let mut rev_offset = 0;
    let rev = chars.0.chars().rev().try_for_each(|x| {
        if x.is_numeric() {
            rev_offset += 1;
            return Ok(())
        }
        Err(())
    });
    let mut forward_offset = 0;
    let forward = chars.1.chars().try_for_each(|x| {
        if x.is_numeric() {
            forward_offset += 1;
            return Ok(())
        }
        Err(())
    });
    (0, 0)
}
fn check_neighborhood(loc: (usize, usize), schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    for r  in -1..1 {
        for c in -1..1 {
            if r < 0 || r >= schematic.len() as i32 {
                continue;
            }
            if c <= 0 || c >= schematic[0].len() as i32 {
                continue;
            }
            if schematic[r as usize][c as usize].is_numeric() {
                neighbors.push((r as usize, c as usize));
            }
        }
    }
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
}