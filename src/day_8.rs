use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(l: &str) -> Self {
        parse_node(l)
    }
}
struct Nodes {
    nodes: BTreeMap<String, Node>,
}
impl Nodes {
    fn next(&self, d: char) -> Option<Node> {
        self.nodes.get()
    }
}
fn parse_file(fname: &str) -> (String, BTreeMap<String, (String, String)>) {
    let file = File::open(fname).unwrap();
    let mut lines = BufReader::new(file).lines();
    let instructions = lines.next().unwrap().unwrap();
    lines.next();
    let btree = lines
        .into_iter()
        .map(|x| parse_line(x.unwrap().as_str()))
        .collect();
    (instructions, btree)
}
fn parse_line(l: &str) -> (String, (String, String)) {
    let mut l = l.split(" = ");
    let address = l.next().unwrap().to_string();
    let lr = &l.next().unwrap()[1..9];
    let mut lr_split = lr.split(", ");
    let left = lr_split.next().unwrap().to_string();
    let right = lr_split.next().unwrap().to_string();
    (address, (left, right))
}
fn parse_node(l: &str) -> Node {
    let mut l = l.split(" = ");
    let address = l.next().unwrap().to_string();
    let lr = &l.next().unwrap()[1..9];
    let mut lr_split = lr.split(", ");
    let left = lr_split.next().unwrap().to_string();
    let right = lr_split.next().unwrap().to_string();
    Node {
        value: address,
        left,
        right,
    }
}
fn find_steps(fname: &str) {
    let (instructions, nodes) = parse_file(fname);
    println!("{:?}", nodes);
    println!("{}", nodes.get("AAA").unwrap().0);
    let mut current = &"AAA".to_string();
    let end = &"ZZZ".to_string();
    let mut steps: u32 = 0;
    while current != end {
        _ = instructions.chars().try_for_each(|d| {
            let c = nodes.get(current.as_str()).unwrap();
            match d {
                'R' => {
                    current = &c.1;
                    steps += 1
                }
                'L' => {
                    current = &c.0;
                    steps += 1
                }
                _ => panic!("Shouldn't be here"),
            };
            return match current == end {
                true => Err(()),
                false => Ok(()),
            };
        });
    }
    println!("steps: {}", steps);
}
fn next<'a>(points: &mut (String, &(String, String)), direction: char, nodes: &'a BTreeMap<String, (String, String)>) -> Option<(&'a String, &'a (String, String))> {
        let node = nodes.get(&points.0).unwrap();
        return match direction {
                'R' => {
                    let new_node = nodes.get_key_value(&node.1).unwrap();
                    match new_node.0.chars().last().unwrap() == 'Z' {
                        true => {None}
                        false => {Some(new_node)}
                    }
                }
                'L' => {
                    let new_node = nodes.get_key_value(&node.0).unwrap();
                    match new_node.0.chars().last().unwrap() == 'Z' {
                        true => {None}
                        false => {Some(new_node)}
                    }
                }
                _ => panic!("Shouldn't be here"),
            }
    }

fn compare(points: &Vec<String>, ) -> bool {
     match points.iter().try_for_each(|x| {
        match x.chars().last().unwrap() == 'Z' {
            true => {Ok(())}
            false => {Err(())}
        }
    }) {
         Ok(_) => {true}
         Err(_) => {false}
     }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_8::{find_steps, parse_file};

    #[test]
    fn test_part_1() {
        find_steps("input/day_8_input.txt");
    }
    #[test]
    fn test_part_1_example_1() {
        find_steps("input/day_8_example_1.txt");
    }
    #[test]
    fn test_part_1_example_2() {
        find_steps("input/day_8_example_2.txt");
    }
    #[test]
    fn find_nodes_end_with_a() {
        let (instructions, nodes) = parse_file("input/day_8_example_1.txt");
        let mut starting_points: Vec<&(String, &(String, String))> = nodes.iter().filter(|(k, v)| {k.chars().last().unwrap() == 'A'}).collect();
        let route_lengths = starting_points.iter().map(|x| {
            let mut stop = false;
            while !stop {
                match next(x, )
            }
        }).collect();

    }
}
