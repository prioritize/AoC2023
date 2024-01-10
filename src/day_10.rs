use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::Add;
use std::ops::Add;

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
    pipes: HashMap<Posit, Segments>,
    visited: HashMap<Posit, bool>,
    starting: (usize, usize),
    dims: (usize, usize),
}
impl Layout {
    fn new(fname: &str) -> Self {
        // let mut pipes = BTreeMap::new();
        // let segments = parse(fname);
        // let mut pipes: BTreeMap<(usize, usize), Segments> = segments
        //     .collect();

        todo!();
    }
    fn neighbors(&self, p: Posit) -> Vec<Posit> {
        let offsets = vec![Posit::new(-1, 0), Posit::new(1, 0), Posit::new(0, -1), Posit::new(0, 1)];
        let posits: Vec<Posit> = offsets.iter().map(|x| {
            return x.clone() + p
        }).collect().iter().filter(|x|{x.check_valid()});
    }
}
impl Add for Posit {
    type Output = (Posit);
    fn add(self, rhs: Self) -> Self::Output {
        Posit::new(self.r+rhs.r, self.c + rhs.c)
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Posit {
    r: i32,
    c: i32,
}
impl Display for Posit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Posit ({}, {})", self.r, self.c)
    }
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
fn process(pipeline: HashMap<Posit, Segments>, start: Posit, dims: (i32, i32)) {
    let mut current = start;
    let mut counter = 0;
    let mut stack = vec![];
    let mut seen_start = false;
    let mut visited = pipeline.iter().map(|(p, s) |{
        (p.clone(), false)
    }).collect();
    loop {
        // stack.append(&mut current.neighbors(&visited, dims));
        *visited.get_mut(&current).unwrap() = true;
        counter+=1;
        match pipeline.get(&current) {
            None => {
                panic!("Shouldn't get here");
            }
            Some(s) => {
                match s {
                    Segments::Start => {
                        if seen_start == false {
                            seen_start = true;
                        } else {
                            break
                        }
                    }
                    _ => {}
                }
            }
        }
        current = stack.pop().unwrap();
        println!("counter: {}", counter);
    }

}
fn parse(fname: &str) -> (HashMap<Posit, Segments>, Posit, (i32, i32)) {
    let file = File::open(fname).expect(&format!("unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    let mut start: Posit = Posit { r: 0, c: 0 };
    let mut entries: Vec<Vec<(Posit, Segments)>> = lines
        .enumerate()
        .map(|(r_idx, l)| {
            return l
                .unwrap()
                .chars()
                .enumerate()
                .map(|(c_idx, c)| match c {
                    '|' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::Pipe),
                    '-' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::Dash),
                    'L' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::L),
                    'J' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::J),
                    'F' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::F),
                    '7' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::Seven),
                    'S' => {
                        let start_loc = (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::Start);
                        start = start_loc.0.clone();
                        start_loc
                    }
                    '.' => (Posit { r: r_idx as i32, c: c_idx as i32 }, Segments::Dot),
                    _ => {
                        panic!("Shouldn't get here")
                    }
                })
                .collect();
        })
        .collect();
    let dims = (entries.len() as i32, entries[0].len() as i32);
    (
        entries
            .into_iter()
            .flatten()
            .collect::<Vec<(Posit, Segments)>>()
            .into_iter()
            .collect(),
        start,
        dims
    )
}
impl Posit {
    // TODO: This function should only generate 4 possible neighbors, as diagonals aren't valid
    // TODO: Consider checking the values here and validating that the path can continue (has a valid direction/ character)
    fn check_valid(&self, dims: (i32, i32)) -> bool {
        if self.r < 0 || self.c < 0 {
            return false
        }
        if self.r >= dims.0 || self.c >= dims.1 {
            return false
        }
        true
    }
    // fn neighbors(&self, visited: &HashMap<Posit, bool>, dims: (i32, i32)) -> Vec<Posit> {
        // let mut out = vec![];
        // let offsets = vec![(-1, 0), (1, 0)]
        // vec![Posit::new(self.r-1, self.c), Posit::new(self.r+1, self.c), Posit::new(self.r)]
        // for r in -1..2 {
        //     for c in -1..2 {
        //         let r_idx = self.r + r;
        //         let c_idx = self.c + c;
        //         if r_idx == self.r && c_idx == self.c {
        //             continue
        //         }
        //         match r_idx < dims.0 && r_idx >= 0 && c_idx < dims.1 && c_idx >= 0 {
        //             true => {
        //                 let p = Posit{r: r_idx, c: c_idx};
        //                 match visited.get(&p).unwrap() {
        //                     false => {out.push(p)}
        //                     true => {}
        //                 }
        //             }
        //             false => {}
        //         }
        //     }
        // }
        // out
    // }
    fn new(r: i32, c: i32) -> Self {
        Posit {r, c}

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let segments = parse("input/day_10_input.txt");
        println!("{:?}", segments.1);
    }
    // #[test]
    // fn test_neighbors() {
    //     let segments = parse("input/day_10_input.txt");
    //     let visited = segments.0.iter().map(|(p, s) |{
    //         (p.clone(), false)
    //     }).collect();
    //     let p = Posit{r: 0, c: 0};
    //     let n = p.neighbors(&visited, segments.2);
    //     let answer = vec![Posit{r:0, c:1}, Posit{r:1, c:0}, Posit{r: 1, c:1}];
    //     assert_eq!(answer, n);
    //     let p = Posit::new(10, 10);
    //     let n = p.neighbors(&visited, segments.2);
    //     let answer = vec![
    //         Posit{r: 9, c: 9}, Posit{r:9, c:10}, Posit{r:9, c:11},
    //         Posit{r: 10, c: 9}, Posit{r:10, c:11},
    //         Posit{r: 11, c: 9}, Posit{r:11, c:10}, Posit{r:11, c:11},
    //     ];
    //     assert_eq!(answer, n);
    // }
    // #[test]
    // fn test_process() {
    //     let segments = parse("input/day_10_input.txt");
    //     process(segments.0, segments.1, segments.2);
    //
    // }
}
