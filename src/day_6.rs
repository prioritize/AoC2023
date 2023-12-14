use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::Sub;

fn parse_file(fname: &str) -> Vec<(u32, u32)> {
    let file = File::open(fname).unwrap();
    let mut lines = BufReader::new(file).lines();
    let fl = lines.next().unwrap().unwrap();
    let sl = lines.next().unwrap().unwrap();
    let mut fl = fl.split(":");
    let mut sl = sl.split(":");
    fl.next();
    sl.next();
    let times: Vec<u32> = fl
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let distances: Vec<u32> = sl
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    zip(times, distances).collect()
}
fn generate_table(range_max: u32) -> Vec<u32> {
    (1..range_max)
        .into_iter()
        .map(|x| {
            let x = u32::try_from(x).unwrap();
            let out = x * (range_max.sub(x));
            // println!("{} * {} = {}", x, (range_max.sub(x)), out);
            out
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let output = parse_file("input/day_6_input.txt");
        assert_eq!(output, vec![(52, 426), (94, 1374), (75, 1279), (94, 1216)]);
    }
    #[test]
    fn test_how_many() {
        let table = generate_table(52);
        let count: Vec<bool> = table
            .iter()
            .map(|x| x > &426)
            .filter(|x| x == &true)
            .collect();
        println!("count: {}", count.len());
    }
    #[test]
    fn test_part_1() {
        let mut buffer: u32 = 1;
        let output = parse_file("input/day_6_input.txt");
        _ = output
            .iter()
            .map(|(t, d)| {
                let table = generate_table(*t);
                let count = table
                    .iter()
                    .map(|x| x > d)
                    .filter(|x| x == &true)
                    .collect::<Vec<bool>>()
                    .len();
                buffer = buffer * u32::try_from(count).unwrap();
                // count
            })
            .collect::<Vec<_>>();
        println!("buffer: {}", buffer)
    }
    #[test]
    fn test_part_2() {
        let t: f64 = 52_947_594.0;
        let d: f64 = 426_137_412_791_216.0;
        let lower = 0.5 * (t - (t.powf(2.0) - 4.0 * d).sqrt());
        let upper = 0.5 * ((t.powf(2.0) - 4.0 * d).sqrt() + t);
        let mut l_u = lower as u64;
        let mut u_u = upper as u64;
        let lower_bound = l_u * (t as u64 - l_u);
        let upper_bound = u_u * (t as u64 - u_u);
        println!("Upper Bound > Record Distance: {}", upper_bound > d as u64);
        println!("Upper: {}: -- D: {}", upper_bound, d as u64);
        println!("Lower Bound > Record Distance: {}", lower_bound > d as u64);
        println!("Lower: {}: -- D: {}", lower_bound, d as u64);
        let u_chances = t as u64 - u_u;
        let possible_paths = t as u64 - (l_u + u_chances);
        println!("{}, {}", l_u, u_u);
        println!("Possible Victory Paths: {}", possible_paths);
    }
}
