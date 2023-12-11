use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn parse_file(fname: &str) {
    let file = File::open(fname).unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut first_line = lines.next().unwrap().unwrap();
    let mut fl_split = first_line.split(":");
    let seeds_word = fl_split.next().unwrap();
    let seeds_actual = fl_split
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .into_iter()
        .map(|x| {
            println!("{}", x);
            x.parse().unwrap()
        })
        .collect::<Vec<u32>>();
    println!("{:?}", seeds_word);
    println!("{:?}", seeds_actual);
    // Throw away empty line
    lines.next();
    let mut remaining = lines
        .into_iter()
        .flatten()
        .collect::<VecDeque<String>>();
    let mut current = vec![];
    let mut mappings = vec![];
    println!("{:?}", remaining);
    while remaining.len() != 0 {
       match remaining.pop_front() {
           None => {
               mappings.push(current.clone());
               current.clear();
               remaining.pop_front();
           }
           Some(value) => {
               if value == "" {
                   mappings.push(current.clone());
                   current.clear();
               } else {
                   println!("{}", value);
                   current.push(value);
               }
           }
       }
    }
    println!("{:?}", mappings);
    // parse_maps(&mut remaining);
}

struct GardenMap {
    source: String,
    destination: String,
    ranges: Vec<(Range<u32>, Range<u32>)>,
}
impl GardenMap {
    fn new(header: String, ranges: Vec<String>) -> Self {
        let mut header = header.split(" ");
        let mut info = header.next().unwrap().split("-");
        let source = info.next().unwrap();
        // Throw away the "to"
        info.next();
        let dest = info.next().unwrap();

        let ranges = ranges
            .iter()
            .map(|l| {
                let mut mapping = l.split(" ");
                let source_start = mapping.next().unwrap().parse().unwrap();
                let dest_start = mapping.next().unwrap().parse().unwrap();
                let offset: u32 = mapping.next().unwrap().parse().unwrap();
                (
                    source_start..source_start + offset,
                    dest_start..dest_start + offset,
                )
            })
            .collect::<Vec<(Range<u32>, Range<u32>)>>();
        let source = String::from(source);
        let destination = String::from(dest);
        GardenMap {
            source,
            destination,
            ranges,
        }
    }
}
fn parse_map(section: &mut VecDeque<String>) -> GardenMap {
    let front = section.pop_front().unwrap();
    let mut split = front.split("-");
    let source = split.next().unwrap().to_string();
    let destination = split.next().unwrap().to_string();
    let mappings: Vec<(Range<u32>, Range<u32>)> = section.iter().map(|l| {
        let values = l.split(" ").into_iter().map(|v| {v.parse().unwrap()}).collect::<Vec<Vec<u32>>>();
        values.iter().map(|v| {(v[0]..v[0]+v[2], v[1]..v[1]+v[2])}).collect::<Vec<(Range<u32>, Range<u32>)>>()
    }).collect();


}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_file() {
        parse_file("input/day_5_example.txt");
    }
}
