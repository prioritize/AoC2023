use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

#[derive(Debug)]
struct OverallMap {
    big_map: HashMap<Stage, GardenMap>,
}
#[derive(Debug)]
struct GardenMap {
    source: Stage,
    destination: Stage,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}
#[derive(Debug, PartialEq)]
struct Seed {
    current: u64,
    stage: Stage,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Stage {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
impl Iterator for Stage {
    type Item = Stage;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Stage::Seed => Some(Stage::Soil),
            Stage::Soil => Some(Stage::Fertilizer),
            Stage::Fertilizer => Some(Stage::Water),
            Stage::Water => Some(Stage::Light),
            Stage::Light => Some(Stage::Temperature),
            Stage::Temperature => Some(Stage::Humidity),
            Stage::Humidity => Some(Stage::Location),
            Stage::Location => None,
        }
    }
}
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Seed => {
                write!(f, "Seed")
            }
            Stage::Soil => {
                write!(f, "Soil")
            }
            Stage::Fertilizer => {
                write!(f, "Fertilizer")
            }
            Stage::Water => {
                write!(f, "Water")
            }
            Stage::Light => {
                write!(f, "Light")
            }
            Stage::Temperature => {
                write!(f, "Temperature")
            }
            Stage::Humidity => {
                write!(f, "Humidity")
            }
            Stage::Location => {
                write!(f, "Location")
            }
        }
    }
}
fn parse_file(fname: &str) -> (Vec<Seed>, OverallMap) {
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
        .collect::<Vec<u64>>();
    let seeds = seeds_actual
        .iter()
        .map(|x| Seed {
            current: x.clone(),
            stage: Stage::Seed,
        })
        .collect();
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
    mappings.push(current.clone());
    let mut garden_maps: Vec<GardenMap> = mappings.iter_mut().map(|m| GardenMap::new(m)).collect();
    let garden_maps: HashMap<Stage, GardenMap> = garden_maps
        .into_iter()
        .map(|m| (m.source.clone(), m))
        .collect();
    let overall_map = OverallMap {
        big_map: garden_maps,
    };
    (seeds, overall_map)
}

impl Seed {
    fn next(&mut self, map: &OverallMap) -> Option<&Self> {
        return match self.stage {
            Stage::Location => None,
            _ => {
                self.iterate(&map);
                self.stage = self.stage.next().unwrap();
                return Some(self);
            }
        };
    }
    fn iterate(&mut self, map: &OverallMap) {
        let r = map.get_range(&self);
        match r {
            None => {
                self.current = self.current;
            }
            Some((source, dest)) => {
                let offset = self.current - source.start;
                self.current = dest.start + offset;
            }
        }
        // self.stage = self.stage.next().unwrap();
    }
}

impl OverallMap {
    fn get_range(&self, seed: &Seed) -> Option<&(Range<u64>, Range<u64>)> {
        return match self
            .big_map
            .get(&seed.stage)
            .unwrap()
            .ranges
            .iter()
            .find(|x| x.0.contains(&seed.current))
        {
            None => None,
            Some(r) => Some(r),
        };
    }
}
impl GardenMap {
    fn new(section: &mut VecDeque<String>) -> Self {
        let front = section.pop_front().unwrap();
        let mut split = front.split("-");
        let source = split.next().unwrap().to_string();
        split.next();
        let destination = split.next().unwrap().split(" ").next().unwrap().to_string();
        let mappings: Vec<(Range<u64>, Range<u64>)> = section
            .iter()
            .map(|l| {
                let v = l
                    .split(" ")
                    .into_iter()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<u64>>();
                (v[1]..v[1] + v[2], v[0]..v[0] + v[2])
            })
            .collect();
        GardenMap {
            source: match_map(&source),
            destination: match_map(&destination),
            ranges: mappings,
        }
    }
}
fn match_map(input: &str) -> Stage {
    return match input {
        "seed" => Stage::Seed,
        "soil" => Stage::Soil,
        "fertilizer" => Stage::Fertilizer,
        "water" => Stage::Water,
        "light" => Stage::Light,
        "temperature" => Stage::Temperature,
        "humidity" => Stage::Humidity,
        "location" => Stage::Location,
        _ => panic!("Got an invalid string"),
    };
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_file() {
        parse_file("input/day_5_example.txt");
    }

    #[test]
    fn test_get_next_seed() {
        let (mut seeds, map) = parse_file("input/day_5_example.txt");
        while seeds[1].next(&map) != None {
            println!("{:?}", seeds[0]);
        }
    }
    #[test]
    fn test_example_part_1() {
        let (mut seeds, map) = parse_file("input/day_5_example.txt");
        for mut s in seeds {
            while s.next(&map) != None {
                println!("{:?}", s);
            }
            println!("\n\n");
        }
    }
    #[test]
    fn test_stage_next() {
        let mut stage = Stage::Seed;
        println!("{}", stage.next().unwrap())
    }
    #[test]
    fn test_part_1() {
        let (mut seeds, map) = parse_file("input/day_5_input.txt");
         let m: u64 =  seeds.iter_mut().map(|x| {
            while x.next(&map) != None {}
            x.current
        }).collect::<Vec<u64>>().iter().min().unwrap().clone();
        println!("{}", m);
    }
    #[test]
    fn test_part_2() {
        let (mut seeds, map) = parse_file("input/day_5_input.txt");
        let file = File::open("input/day_5_input.txt").unwrap();
        let mut lines = BufReader::new(file).lines();
        let first_line = lines.next().unwrap().unwrap();
        let mut split = first_line.split(":");
        split.next();
        let seed_info = split.next().unwrap().trim();
        let mut seed_info = seed_info.split(" ").into_iter().collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect::<VecDeque<u64>>();
        let mut ranges = vec![];
        while seed_info.len() != 0 {
            ranges.push((seed_info.pop_front().unwrap(), seed_info.pop_front().unwrap()));
        }
        let ranges: Vec<Range<u64>> = ranges.iter().map(|(start, span)|{start.clone()..start.clone()+span}).collect();
        let mut smallest = u64::MAX;
        ranges.iter().for_each(|x| {
            for s in x.clone().into_iter() {
                let mut seed = Seed{current: s, stage: Stage::Seed};
                while seed.next(&map) != None {}
                if seed.current < smallest {
                    smallest = seed.current;
                }
            }
            println!("Finished a range")
        });
        println!("{}", smallest);

    }
}
