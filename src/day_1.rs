use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn part_1() -> u32 {
    let file = File::open("input/day_1_input.txt").expect("Unable to open input file");
    let mut lines = BufReader::new(file).lines();
    // let mut lines = String::new();
    for line in lines {
        match line {
            Ok(l) => {
                println!("{}", l);
            },
            _ => {}
        }
    }
    8
}
