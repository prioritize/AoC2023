use std::collections::{BTreeMap, HashMap};
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
struct Layout {
    pipes: BTreeMap<Posit, Segments>,
    visited: BTreeMap<Posit, bool>,
    starting: (usize, usize),
}
impl Layout {
    fn new(fname: &str) -> Self {
        // let mut pipes = BTreeMap::new();
        // let segments = parse(fname);
        // let mut pipes: BTreeMap<(usize, usize), Segments> = segments
        //     .collect();

        todo!();
    }
}
struct Posit {
    r: usize,
    c: usize,
}
impl Display for Segments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Segments::L => {
                write!(f, "L")
            }
            Segments::F => {
                write!(f, "F")
            }
            Segments::Pipe => {
                write!(f, "|")
            }
            Segments::Dash => {
                write!(f, "-")
            }
            Segments::J => {
                write!(f, "J")
            }
            Segments::Seven => {
                write!(f, "7")
            }
            Segments::Start => {
                write!(f, "S")
            }
            Segments::Dot => {
                write!(f, ".")
            }
        }
    }
}
fn parse(fname: &str) -> HashMap<(usize, usize), Segments> {
    let file = File::open(fname).expect(&format!("unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    let mut entries: Vec<Vec<((usize, usize), Segments)>> = lines
        .enumerate()
        .map(|(r_idx, l)| {
            return l
                .unwrap()
                .chars()
                .enumerate()
                .map(|(c_idx, c)| match c {
                    '|' => ((r_idx, c_idx), Segments::Pipe),
                    '-' => ((r_idx, c_idx), Segments::Dash),
                    'L' => ((r_idx, c_idx), Segments::L),
                    'J' => ((r_idx, c_idx), Segments::J),
                    'F' => ((r_idx, c_idx), Segments::F),
                    '7' => ((r_idx, c_idx), Segments::Seven),
                    'S' => ((r_idx, c_idx), Segments::Start),
                    '.' => ((r_idx, c_idx), Segments::Dot),
                    _ => {
                        panic!("Shouldn't get here")
                    }
                })
                .collect();
        })
        .collect();
    entries.into_iter().flatten().collect::<Vec<((usize,usize), Segments)>>().into_iter().collect()
}
fn neighbors() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let segments = parse("input/day_10_input.txt");
        println!("{:?}", segments);
    }
}
