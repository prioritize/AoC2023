use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::Sub;

fn parse(fname: &str) -> Vec<Vec<i32>> {
    let file = File::open(fname).expect(&format!("unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    lines
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn difference<T>(incoming: &Vec<T>) -> Vec<T>
where
    T: Sub<Output = T> + Copy + Debug + Display,
{
    let mut idx = 0;
    let mut out: Vec<T> = vec![];
    while idx < incoming.len() - 1 {
        // println!("Subtraction: {} - {} = {}", incoming[idx], incoming[idx+1], incoming[idx] - incoming[idx+1]);
        out.push((incoming[idx + 1].sub(incoming[idx])));
        // println!("{:?}", out);
        idx += 1;
    }
    out
}
fn predict_entry(entry: &mut Vec<i32>) -> i32 {
    let mut stages = iterate_to_zeros(entry);
    predict(&mut stages, entry);
    println!("Extrapolated: {}", entry.last().unwrap());
    entry.last().unwrap().clone()
}
fn is_zeroes(incoming: &Vec<i32>) -> bool {
    let mut trues = 0;
    incoming.iter().for_each(|x| match *x == 0 {
        true => {
            trues += 1;
        }
        false => {}
    });
    return match trues == incoming.len() {
        true => true,
        false => false,
    };
}
fn iterate_to_zeros(incoming: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut depth = 0;
    let mut current = incoming.clone();
    let mut check_current = is_zeroes(&current);
    let mut difference_holder = vec![];
    while !check_current {
        // Calculate the differences
        current = difference(&current);
        // Increment the depth (this might be required for zero padding
        depth += 1;
        difference_holder.push(current.clone());
        check_current = is_zeroes(&current);
    }
    difference_holder
}
fn predict(stages: &mut Vec<Vec<i32>>, values: &mut Vec<i32>) {
    let mut starting_prev = 0;
    let mut current_depth = stages.len();
    let mut storage: Vec<i32> = stages
        .iter()
        .rev()
        .map(|x| {
            let temp = starting_prev + x.last().unwrap();
            starting_prev = temp;
            temp
        })
        .collect();
    storage.reverse();
    let stages: Vec<Vec<i32>> = zip(stages, storage)
        .map(|(x, y)| {
            x.push(y);
            x.clone()
        })
        .collect();
    println!("{:?}", values);
    values.push(values.last().unwrap() + stages.first().unwrap().last().unwrap());
    println!("{:?}", values);
}
fn calculate_extrapolated_values(report: &mut Vec<Vec<i32>>) -> i32 {
    report
        .iter_mut()
        .map(|entry| predict_entry(entry))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let vecs = parse("input/day_9_input.txt");
        let l0 = vec![
            10, 13, 27, 64, 140, 281, 549, 1101, 2294, 4858, 10201, 21027, 42709, 86382, 175728,
            361360, 750462, 1566611, 3268603, 6783010, 13951998,
        ];
        assert_eq!(l0, vecs[0]);
    }
    #[test]
    fn test_difference() {
        let vecs = parse("input/day_9_input.txt");
        let l0 = vec![
            10, 13, 27, 64, 140, 281, 549, 1101, 2294, 4858, 10201, 21027, 42709, 86382, 175728,
            361360, 750462, 1566611, 3268603, 6783010, 13951998,
        ];
        let d0 = vec![
            3, 14, 37, 76, 141, 268, 552, 1193, 2564, 5343, 10826, 21682, 43673, 89346, 185632,
            389102, 816149, 1701992, 3514407, 7168988,
        ];
        assert_eq!(l0, vecs[0]);
        assert_eq!(d0, difference(&vecs[0]));
    }
    #[test]
    fn test_determine_zeroes() {
        let vecs = parse("input/day_9_input.txt");
        let differences = difference(&vecs[0]);
        assert_eq!(false, is_zeroes(&differences));
        let differences = difference(&differences);
        assert_eq!(false, is_zeroes(&differences));
        let differences = difference(&differences);
        assert_eq!(false, is_zeroes(&differences));
    }
    #[test]
    fn test_determine_zeroes_example() {
        let vecs = parse("input/day_9_example.txt");
        let differences = difference(&vecs[0]);
        assert_eq!(false, is_zeroes(&differences));
        let differences = difference(&differences);
    }
    #[test]
    fn test_iterate() {
        let vecs = parse("input/day_9_input.txt");
        let stages = iterate_to_zeros(&vecs[0]);
    }
    #[test]
    fn test_predict() {
        let mut vecs = parse("input/day_9_input.txt");
        let total = calculate_extrapolated_values(&mut vecs);
        println!("total: {}", total);
    }
    #[test]
    fn test_predict_future() {
        let mut vecs = parse("input/day_9_input.txt");
        vecs.iter_mut().for_each(|x| x.reverse());
        let total = calculate_extrapolated_values(&mut vecs);
        println!("total: {}", total);
    }
    #[test]
    fn test_predict_example() {
        let mut vecs = parse("input/day_9_example.txt");
        let total = calculate_extrapolated_values(&mut vecs);
        println!("total: {}", total);
    }
    #[test]
    fn test_predict_example_future() {
        let mut vecs = parse("input/day_9_example.txt");
        vecs.iter_mut().for_each(|x| x.reverse());
        let total = calculate_extrapolated_values(&mut vecs);
        println!("total: {}", total);
    }
}
