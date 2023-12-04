use aho_corasick::{AhoCorasick, PatternID};
use itertools::Itertools;
use regex::Match;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(fname: &str) -> u32 {
    let file = File::open(fname).expect("Unable to open input file");
    let lines = BufReader::new(file).lines();
    let out = lines
        .map(|l| match l {
            Ok(l) => {
                let first = l.chars().find_or_first(|c| c.is_numeric()).unwrap();
                let last = l.chars().rev().find_or_last(|c| c.is_numeric()).unwrap();
                first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
            }
            Err(_) => 0,
        })
        .sum();
    println!("{}", out);
    out
}
pub fn part_2(fname: &str) -> u32 {
    let re_for = r"(one|two|three|four|five|six|seven|eight|nine)";
    let re = regex::Regex::new(re_for).unwrap();
    let file = File::open(fname).expect("Unable to open input file");
    let lines = BufReader::new(file).lines();

    let out = lines
        .map(|l| match l {
            Ok(l) => {
                let first: u32;
                let last: u32;
                let numerals = l
                    .char_indices()
                    .filter(|(_, c)| c.is_numeric())
                    .collect::<Vec<(usize, char)>>();
                let matches = re.find_iter(l.as_str()).collect::<Vec<Match>>();
                first = check_numeral_or_word(matches.first(), numerals.first(), less);
                last = check_numeral_or_word(matches.last(), numerals.last(), greater);
                first * 10 + last
            }
            Err(_) => 0,
        })
        .sum();
    println!("{}", out);
    out
}
pub fn day_1_part_2_non_regex(fname: &str) {
    let file = File::open(fname).expect("Unable to open input file");
    let lines = BufReader::new(file).lines();
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let line_matches = lines
        .map(|l| {
            let mut matches = vec![];
            let stupid = l.unwrap();
            let _ = ac.find_overlapping_iter(stupid.as_str()).for_each(|m| {
                matches.push((m.pattern(), m.start(), m.end()));
            });
            matches.clone()
        })
        .collect::<Vec<Vec<(PatternID, usize, usize)>>>();
    let out: u32 = line_matches
        .iter()
        .map(|m| {
            match_word_to_u32(patterns[m.first().unwrap().0]) * 10
                + match_word_to_u32(patterns[m.last().unwrap().0])
        })
        .sum();
    println!("{}", out);
}
fn check_numeral_or_word(
    matches: Option<&Match>,
    numerals: Option<&(usize, char)>,
    comp: fn(usize, usize) -> bool,
) -> u32 {
    return match matches {
        None => {
            numerals.unwrap().1.to_digit(10).unwrap()
        }

        Some(m) => match numerals {
            Some(n) => {
                if comp(m.start(), n.0) {
                    match_word_to_u32(m.as_str())
                } else {
                    n.1.to_digit(10).unwrap()
                }
            }
            None => match_word_to_u32(m.as_str()),
        },
    };
}
fn greater(l: usize, r: usize) -> bool {
    l > r
}
fn less(l: usize, r: usize) -> bool {
    l < r
}

fn match_word_to_u32(input: &str) -> u32 {
    return match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => {
            panic!("Bad input passed to string_to_number")
        }
    };
}
