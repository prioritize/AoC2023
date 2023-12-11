use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
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
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    // Throw away empty line
    lines.next();
    let mut remaining = lines.into_iter().flatten().collect::<VecDeque<String>>();
    let mut current = VecDeque::new();
    let mut mappings = vec![];
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
                    current.push_back(value);
                }
            }
        }
    }
    let mut garden_maps: Vec<GardenMap> = mappings.iter_mut().map(|m| GardenMap::new(m)).collect();
    let garden_maps: HashMap<String, GardenMap> = garden_maps
        .into_iter()
        .map(|m| (m.source.clone(), m))
        .collect();
    let overall_map = OverallMap{big_map:garden_maps};
    println!("{}", overall_map.evaluate_seed("seed", 79));
    // println!("{:?}", garden_maps.get("soil").unwrap())
}

#[derive(Debug)]
struct GardenMap {
    source: String,
    destination: String,
    ranges: Vec<(Range<u32>, Range<u32>)>,
}
struct OverallMap {
    big_map: HashMap<String, GardenMap>,
}
impl OverallMap {
    fn evaluate_seed(&self, source: &str, seed: u32) -> u32 {
        let seed_range: Vec<&(Range<u32>, Range<u32>)> = self
            .big_map
            .get(source)
            .unwrap()
            .ranges
            .iter()
            .map(|x| match x.0.contains(&seed) {
                true => {Some(x)}
                false => {None}
            }).flatten()
            .collect();
        let mut mapped_value= 0;
        match seed_range.len() {
            0 => {
               mapped_value = seed;
                println!("Found None: {}", mapped_value);
            }
            1 => {
                println!("Seed Range: {:?}", seed_range);
                let seed_range = seed_range[0];
                println!("Seed Range: {:?}", seed_range);
                let offset = seed - seed_range.0.start;
                println!("Offset: {:?}", offset);
                mapped_value = seed_range.1.start + offset;
            }
            _ => panic!("shouldn't be getting more than 1")
        }
        println!("{}", self.big_map.get(source).unwrap().destination);
        println!("")
        match self.big_map.get(source).unwrap().destination.as_str() {
            "soil" => self.evaluate_seed("soil", mapped_value),
            "fertilizer" => self.evaluate_seed("fertilizer", mapped_value),
            "water" => self.evaluate_seed("water", mapped_value),
            "light" => self.evaluate_seed("light", mapped_value),
            "temperature" => self.evaluate_seed("temperature", mapped_value),
            "humidity" => self.evaluate_seed("humidity", mapped_value),
            "location" => mapped_value,
            _ => {
                panic!("We shouldn't find ourselves here")
            }
        }
    }
}
impl GardenMap {
    fn new(section: &mut VecDeque<String>) -> Self {
        let front = section.pop_front().unwrap();
        let mut split = front.split("-");
        let source = split.next().unwrap().to_string();
        split.next();
        let destination = split.next().unwrap().split(" ").next().unwrap().to_string();
        let mappings: Vec<(Range<u32>, Range<u32>)> = section
            .iter()
            .map(|l| {
                let v = l
                    .split(" ")
                    .into_iter()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<u32>>();
                println!("{} {} {}", v[0], v[1], v[2]);
                (v[1]..(v[1] + v[2]), v[0]..(v[0] + v[2]))
            })
            .collect();
        GardenMap {
            source,
            destination,
            ranges: mappings,
        }
    }
    // fn evaluate_seed_location(&self, location: u32, source_type: &str) -> u32 {
    //     //     Start with seed, iterate to location
    //      match self.ranges.iter().find(|x| x.0.contains(&seed)) {
    //          None => {}
    //          Some(_) => {}
    //      }
    //
    //     todo!()
    // }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_file() {
        parse_file("input/day_5_example.txt");
    }
}
