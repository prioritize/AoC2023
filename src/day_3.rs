use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

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
    let mut visited = vec![];
    for line in &schematic {
        let mut v = vec![];
        for c in line.iter() {
            v.push(false);
        }
        visited.push(v);
    }

    // Iterate the vectors to find all values that aren't numeric or periods
    // let mut symbols = vec![];
    for (r_idx, r)  in schematic.iter().enumerate() {
        for (c_idx, c) in r.iter().enumerate() {
            if is_symbol(*c) {
                let neighbors = build_neighborhood((r_idx, c_idx));
                let part_numbers = neighbors.iter().map(|x|{
                    if schematic[x.0][x.1].is_numeric() && visited[x.0][x.1] != true {
                        let span = find_start_and_end((x.0, x.1), &schematic);
                        visited[x.0][span.0..span.1].iter_mut().for_each(|mut x| *x = true);
                        //TODO: Convert to finding just the front of the number, and handle passing the correct line number as well
                        Some(concat(&schematic[r_idx][span.0..span.1]))
                        // Some(span)
                    } else { None }
                }).filter(Option::is_some).map(|x| {x.unwrap()});
                for item in part_numbers {
                    println!("{:?}", item)
                }
            }
        }

    }
}

fn concat(vec: &[char]) -> u32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem.to_digit(10).unwrap())
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
fn find_start_and_end(loc: (usize, usize), line: &Vec<Vec<char>>) -> (usize, usize){
    let line = &line[loc.0];
    let chars = line.split_at(loc.1);
    let mut rev_offset: usize = 0;
    let rev = chars.0.iter().rev().try_for_each(|x| {
        if x.is_numeric() {
            rev_offset += 1;
            return Ok(())
        }
        Err(())
    });
    let mut forward_offset = 0;
    let forward = chars.1.iter().try_for_each(|x| {
        if x.is_numeric() {
            forward_offset += 1;
            return Ok(())
        }
        Err(())
    });
    println!("{:?}", &line[loc.1.saturating_sub(rev_offset)..loc.1 + forward_offset]);
    (loc.1 - rev_offset, loc.1 + forward_offset)
}
fn build_neighborhood (loc: (usize, usize), ) -> HashSet<(usize, usize)>{
    let mut neighbors = HashSet::new();
    for r in -1..2 {
        let r_idx = loc.0.saturating_add_signed(r);
        for c in -1..=1 {
            let c_idx = loc.1.saturating_add_signed(c);
            neighbors.insert((r_idx, c_idx));
        }
    }
    println!("location: {}, {} - neighbors: {:?}", loc.0, loc.1, neighbors);
    neighbors
}
fn check_neighborhood(loc: (usize, usize), schematic: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let neighbors = build_neighborhood(loc);
    neighbors.iter().for_each(|x| {
        println!("{:?}", x)
    });
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