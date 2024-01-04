use itertools::{iterate, Itertools};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

type Game = Vec<(u32, HashSet<u32>, HashSet<u32>)>;

fn parse_input(fname: &str) -> Game {
    let file = File::open(fname).expect(format!("couldn't open {}", fname).as_str());
    BufReader::new(file)
        .lines()
        .filter(Result::is_ok)
        .flatten()
        .map(|x| {
            let mut g = x.split(":");
            let front = g.next().unwrap();
            let mut f = front.split_whitespace();
            // Throw away the 'Card'
            f.next();
            let card_number = f.next().unwrap();
            let numbers = g.next().unwrap();
            let mut numbers_split = numbers.split("|");
            let winning_char = numbers_split.next().unwrap();
            let game_numbers = numbers_split.next().unwrap();
            let winning_numbers = winning_char
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<HashSet<u32>>();
            let game_numbers = game_numbers
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<HashSet<u32>>();

            (card_number.parse().unwrap(), winning_numbers, game_numbers)
        })
        .collect::<Game>()
}
fn check_numbers(winners: &HashSet<u32>, chances: &HashSet<u32>) -> HashSet<u32> {
    winners
        .iter()
        .map(|x| {
            return match chances.contains(x) {
                true => Some(*x),
                false => None,
            };
        })
        .flatten()
        .collect()
}
fn day_4_part_1(fname: &str) {
    let cards = parse_input(fname);
    let results = cards
        .iter()
        .map(|(_, winners, chances)| check_numbers(winners, chances))
        .filter(|w| w.len() != 0)
        .map(|x| 2_u32.pow(x.len() as u32 - 1))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("{}", results)
}
fn day_4_part_2(fname: &str) {
    let cards = parse_input(fname);
    let results = cards
        .iter()
        .map(|(c, w, chance)| (usize::try_from(*c).unwrap(), check_numbers(w, chance).len()))
        .collect::<Vec<(usize, usize)>>();
    let results = results
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), RefCell::new(1)))
        .collect::<Vec<(usize, usize, RefCell<u32>)>>();
    let card_counts = results
        .iter()
        .map(|(card, set, count)| {
            let start = card.clone();
            let end = start + set;
            for _ in 0..count.borrow().clone() {
                &results[start..end]
                    .iter()
                    .for_each(move |(c, _, next_count)| {
                        let nc = next_count.clone().take();
                        next_count.replace(nc + 1);
                    });
            }
            count.take()
        })
        .collect::<Vec<u32>>();
    println!("total cards: {}", card_counts.iter().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        parse_input("input/day_4_input.txt");
    }
    #[test]
    fn test_day_4_part_1() {
        day_4_part_1("input/day_4_input.txt");
    }
    #[test]
    fn test_day_4_example() {
        day_4_part_1("input/day_4_example.txt");
    }
    #[ignore]
    #[test]
    fn test_day_4_part_2_example() {
        day_4_part_2("input/day_4_example.txt");
    }
    #[ignore]
    #[test]
    fn test_day_4_part_2() {
        day_4_part_2("input/day_4_input.txt");
    }
}
