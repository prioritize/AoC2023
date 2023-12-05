use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game {
    id: u32,
    hands: Vec<Hand>,
}
impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for item in &self.hands {
            buf.push_str(format!("{}", item).as_str())
        }
        write!(f, "{{ Game: {{id: {} hands: {} }}}}", self.id, buf)
    }
}
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Hand: {{r: {}, g: {}, b: {} }}}}",self.red, self.green, self.blue)
    }
}
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}
fn day_2(fname: &str) {
    let file = File::open(fname).expect(&format!("Unable to open {}", fname));
    let lines = BufReader::new(file).lines();
    let games: Vec<Game> = lines
        .map(|l| {
            let l = l.unwrap();
            let game_number = l.split(":").collect::<Vec<&str>>();
            let number = game_number[0].split(" ").collect::<Vec<&str>>();
            let games = game_number[1].split(";").collect::<Vec<&str>>();
            let cubes = games
                .iter()
                .map(|g| g.split(", ").map(|x| x.trim()).collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            Game {
                id: number[1].parse().unwrap(),
                hands: make_hands(cubes),
            }
        })
        .collect();
    let mut sum = 0;
    for game in &games {
        if is_valid_game(game, 12, 13, 14) {
            sum = sum + game.id;
        }
    }
    let mut power_sum = 0;
    for game in &games {
        power_sum = power_sum + find_min_cubes(game)
    }
    println!("{}", sum);
    println!("{}", power_sum);
}
fn make_hands(input: Vec<Vec<&str>>) -> Vec<Hand> {
    let mut hands = vec![];
    for hand in input {
        let mut current_hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cube in hand {
            let show = cube.split(" ").collect::<Vec<&str>>();
            match show[1] {
                "red" => current_hand.red = show[0].parse().unwrap(),
                "green" => current_hand.green = show[0].parse().unwrap(),
                "blue" => current_hand.blue = show[0].parse().unwrap(),
                _ => {
                    panic!("Shouldn't get here")
                }
            }
        }
        hands.push(current_hand);
    }
    hands
}
fn is_valid_game(game: &Game, r: u32, g: u32, b: u32) -> bool {
    for h in &game.hands {
        if (h.red > r) || (h.green > g) || (h.blue > b) {
            return false
        }
    }
    true
}
fn find_min_cubes(game: &Game) -> u32 {
    let mut rgb = (0, 0, 0);
    for hand in &game.hands {
        if hand.red > rgb.0 {
            rgb.0 = hand.red;
        }
        if hand.green > rgb.1 {
            rgb.1 = hand.green;
        }
        if hand.blue > rgb.2 {
            rgb.2 = hand.blue;
        }
    }
    rgb.0 * rgb.1 * rgb.2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_2_part_1() {
        day_2("input/day_2_input.txt")
    }
}
