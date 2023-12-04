use std::fs::File;
use std::io::{BufRead, BufReader};

fn day_2(fname: &str) {
    let file = File::open(fname).expect(&format!("Unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    let games = lines.for_each(|l|{
        let l = l.unwrap();
        let game_number = l.split(":").collect::<Vec<&str>>();
        let number = game_number[0].split(" ").collect::<Vec<&str>>();
        let games = game_number[1].split(";").collect::<Vec<&str>>();
        let cubes = games.iter().map( | g | {
            g.split(", ").collect::<Vec<&str>>()
        }).collect::<Vec<Vec<&str>>>();
        println!("{:?}", game_number);
        println!("{}", number[1]);
        for game in games {
            println!("games:{}", game);
        }
        for cube in cubes {
            println!("{:?}", cube)
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_2_part_1() {
        day_2("input/day_2_input.txt")
    }
}