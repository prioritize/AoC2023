use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
enum Segments {
    L,
    F,
    Pipe,
    Dash,
    J,
    Seven,
    Start,
    Dot,
}
impl Display for Segments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Segments::L => {write!(f, "L")}
            Segments::F => {write!(f, "F")}
            Segments::Pipe => {write!(f, "|")}
            Segments::Dash => {write!(f, "-")}
            Segments::J => {write!(f, "J")}
            Segments::Seven => {write!(f, "7")}
            Segments::Start => {write!(f, "S")}
            Segments::Dot => {write!(f, ".")}
        }
    }
}
fn parse(fname: &str) -> Vec<Vec<Segments>> {
    let file = File::open(fname).expect(&format!("unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    lines
        .map(|l| {
            return l
                .unwrap()
                .chars()
                .map(|c| match c {
                    '|' => Segments::Pipe,
                    '-' => Segments::Dash,
                    'L' => Segments::L,
                    'J' => Segments::J,
                    'F' => Segments::F,
                    '7' => Segments::Seven,
                    'S' => Segments::Start,
                    '.' => Segments::Dot,
                    _ => {
                        panic!("Shouldn't get here")
                    }
                })
                .collect();
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let segments = parse("input/day_10_input.txt");
        println!("{:?}", segments);
    }
}
