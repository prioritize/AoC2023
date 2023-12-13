use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Iter;

struct OverallMap {
    big_map: HashMap<Stage, GardenMap>,
}
#[derive(Debug)]
struct GardenMap {
    source: Stage,
    destination: Stage,
    ranges: Vec<(Range<u32>, Range<u32>)>,
}
struct Seed {
    current: u32,
    stage: Stage,
}
#[derive(Debug)]
#[derive(Clone, Eq, PartialEq, Hash)]
enum Stage {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}
impl Iterator for Stage {
    type Item = Stage;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Stage::Seed => {Some(Stage::Soil)}
            Stage::Soil => {Some(Stage::Fertilizer)}
            Stage::Fertilizer => {Some(Stage::Water)}
            Stage::Water => {Some(Stage::Light)}
            Stage::Light => {Some(Stage::Temperature)}
            Stage::Temperature => {Some(Stage::Humidity)}
            Stage::Humidity => {Some(Stage::Location)}
            Stage::Location => {None}
        }
    }
}
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stage::Seed => {write!(f, "Seed")}
            Stage::Soil => {write!(f, "Soil")}
            Stage::Fertilizer => {write!(f, "Fertilizer")}
            Stage::Water => {write!(f, "Water")}
            Stage::Light => {write!(f, "Light")}
            Stage::Temperature => {write!(f, "Temperature")}
            Stage::Humidity => {write!(f, "Humidity")}
            Stage::Location => {write!(f, "Location")}
        }
    }
}
fn parse_file(fname: &str) -> (Vec<Seed>, OverallMap){
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
    let seeds = seeds_actual.iter().map(|x| {Seed{current:x.clone(), stage:Stage::Seed}}).collect();
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
    let garden_maps: HashMap<Stage, GardenMap> = garden_maps
        .into_iter()
        .map(|m| (m.source.clone(), m))
        .collect();
    let overall_map = OverallMap {
        big_map: garden_maps,
    };
    (seeds, overall_map)
    // println!("{}", overall_map.evaluate_seed("seed", 79));
}

impl Seed {
    fn next(&mut self, garden_map: &OverallMap) -> Option<Stage> {
        return match self.stage {
            Stage::Seed => {Some(Stage::Soil)}
            Stage::Soil => {Some(Stage::Fertilizer)}
            Stage::Fertilizer => {Some(Stage::Water)}
            Stage::Water => {Some(Stage::Light)}
            Stage::Light => {Some(Stage::Temperature)}
            Stage::Temperature => {Some(Stage::Humidity)}
            Stage::Humidity => {Some(Stage::Location)}
            Stage::Location => {None}
        }
    }
}
impl OverallMap {
    fn get_range(&self, seed: &Seed) -> Option<&(Range<u32>, Range<u32>)> {
        return match self.big_map.get(&seed.stage).unwrap().ranges.iter().find_or_first(|x| {x.1.contains(&seed.current)}) {
            None => {None}
            Some(r) => {Some(r)}
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
            source: match_map(&source),
            destination: match_map(&destination),
            ranges: mappings,
        }
    }
}
fn match_map(input: &str) -> Stage {
    return match input {
        "seed" => {Stage::Seed}
        "soil" => {Stage::Soil}
        "fertilizer" => {Stage::Fertilizer}
        "water" => {Stage::Water}
        "light" => {Stage::Light}
        "temperature" => {Stage::Temperature}
        "humidity" => {Stage::Humidity}
        "location" => {Stage::Location}
        _ => panic!("Got an invalid string")
    }
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
        let s

    }
    #[test]
    fn test_stage_next() {
        let mut stage = Stage::Seed;
        println!("{}", stage.next().unwrap())
    }
}
