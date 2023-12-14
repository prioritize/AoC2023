use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

#[derive(Eq, Debug)]
struct Card {
    value: u32,
}
impl Card {
    fn new(c: char) -> Self {
        match c {
            'A' => Card::gen_new(14),
            'K' => Card::gen_new(13),
            'Q' => Card::gen_new(12),
            'J' => Card::gen_new(11),
            'T' => Card::gen_new(10),
            '9' => Card::gen_new(9),
            '8' => Card::gen_new(8),
            '7' => Card::gen_new(7),
            '6' => Card::gen_new(6),
            '5' => Card::gen_new(5),
            '4' => Card::gen_new(4),
            '3' => Card::gen_new(3),
            '2' => Card::gen_new(2),
            // 'A' => {Card::gen_new(14)}
            _ => panic!("Something bad happened"),
        }
    }
    fn gen_new(v: u32) -> Card {
        Card { value: v }
    }
}
impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}
impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value < other.value {
            true => Some(Ordering::Less),
            false => match self.value > other.value {
                true => Some(Ordering::Greater),
                false => Some(Ordering::Equal),
            },
        }
    }
}
#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    rank: Ranking,
    bid: u32,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //     fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.get_value().unwrap() > other.rank.get_value().unwrap() {
            true => Some(Ordering::Greater),
            false => match self.rank.get_value().unwrap() < other.rank.get_value().unwrap() {
                true => Some(Ordering::Less),
                false => {
                    let s_iter = &self.cards.iter();
                    let mut hand_iter = zip(self.cards.iter(), other.cards.iter());
                    let mut ordering: Option<Ordering> = None;
                    _ = hand_iter.try_for_each(|(s, o)| match s.value == o.value {
                        true => Ok(()),
                        false => {
                            match s.value > o.value {
                                true => ordering = Some(Ordering::Greater),
                                false => ordering = Some(Ordering::Less),
                            }
                            Err(())
                        }
                    });
                    ordering
                }
            },
        }
    }
}
//         ordering
//     }
// }
impl Hand {
    fn new(incoming: &str) -> Self {
        let mut s = incoming.split(" ");
        let hand = s.next().unwrap();
        let cards: Vec<Card> = hand.chars().map(|s| Card::new(s)).collect();
        let bid = s.next().unwrap().parse().unwrap();
        let rank = Hand::determine_ranking(hand);
        Hand { cards, rank, bid }
    }
    fn determine_ranking_with_wilds(hand: &str) -> crate::day_7::Ranking {
        let mut hm = HashMap::new();
        _ = hand
            .chars()
            .into_iter()
            .map(|x| match hm.contains_key(&x) {
                true => {
                    *hm.get_mut(&x).unwrap() += 1;
                }
                false => {
                    hm.insert(x, 1);
                }
            })
            .collect::<Vec<_>>();
        let mut num_j = 0;
        let mut highest: (&char, &u32) = (&' ', &0);
        // let mut highest_count: u32 = 0;
        hm.iter().for_each(|(k, v)| {
            match k == 'J' {
                true => {
                    num_j = v.clone();
                }
                false => {
                    if v > highest.1 {
                        highest = (k, v)
                    }
                }
            }
        });
        if num_j != 0 {
            hm.remove(&'j').unwrap();
            *hm.get_mut(&highest.0).unwrap() += highest.1;
        }
        // The logic here says that based on the number of entries in the hashmap we can
        // determine which type of hand we're dealing with
        return match hm.len() {
            5 => crate::day_7::Ranking::HighCard,
            4 => crate::day_7::Ranking::OnePair,
            3 => match hm.iter().map(|(k, v)| v).max().unwrap() {
                &3 => crate::day_7::Ranking::ThreeOfAKind,
                &2 => crate::day_7::Ranking::TwoPair,
                _ => {
                    panic!("Shouldn't have more than 3 or less than 2")
                }
            },
            2 => {
                match hm.iter().map(|(k, v)| v).max().unwrap() {
                    &4 => crate::day_7::Ranking::FourOfAKind,
                    &3 => crate::day_7::Ranking::FullHouse,
                    _ => {
                        panic!("Shouldn't have more than 4 or less than 3")
                    } //TODO: Split between four of a kind and a full house}
                }
            }
            1 => crate::day_7::Ranking::FiveOfAKind,
            _ => {
                println!("I was unable to determine a hand, this shouldn't happen");
                crate::day_7::Ranking::Undetermined
            }
        }
    }
    fn load_hashmap(hand: &str) -> HashMap<char, u32> {
        let mut hm = HashMap::new();
        _ = hand
            .chars()
            .into_iter()
            .map(|x| match hm.contains_key(&x) {
                true => {
                    *hm.get_mut(&x).unwrap() += 1;
                }
                false => {
                    hm.insert(x, 1);
                }
            })
            .collect::<Vec<_>>();
        hm
    }
    fn determine_hand_type(hm: HashMap<char, u32>) -> Ranking {
        return match hm.len() {
            5 => crate::day_7::Ranking::HighCard,
            4 => crate::day_7::Ranking::OnePair,
            3 => match hm.iter().map(|(k, v)| v).max().unwrap() {
                &3 => crate::day_7::Ranking::ThreeOfAKind,
                &2 => crate::day_7::Ranking::TwoPair,
                _ => {
                    panic!("Shouldn't have more than 3 or less than 2")
                }
            },
            2 => {
                match hm.iter().map(|(k, v)| v).max().unwrap() {
                    &4 => crate::day_7::Ranking::FourOfAKind,
                    &3 => crate::day_7::Ranking::FullHouse,
                    _ => {
                        panic!("Shouldn't have more than 4 or less than 3")
                    } //TODO: Split between four of a kind and a full house}
                }
            }
            1 => crate::day_7::Ranking::FiveOfAKind,
            _ => {
                println!("I was unable to determine a hand, this shouldn't happen");
                crate::day_7::Ranking::Undetermined
            }
        };
    }
}

    fn determine_ranking(hand: &str) -> Ranking {
        let mut hm = Hand::load_hashmap(hand);

        // The logic here says that based on the number of entries in the hashmap we can
        // determine which type of hand we're dealing with
        return match hm.len() {
            5 => Ranking::HighCard,
            4 => Ranking::OnePair,
            3 => match hm.iter().map(|(k, v)| v).max().unwrap() {
                &3 => Ranking::ThreeOfAKind,
                &2 => Ranking::TwoPair,
                _ => {
                    panic!("Shouldn't have more than 3 or less than 2")
                }
            },
            2 => {
                match hm.iter().map(|(k, v)| v).max().unwrap() {
                    &4 => Ranking::FourOfAKind,
                    &3 => Ranking::FullHouse,
                    _ => {
                        panic!("Shouldn't have more than 4 or less than 3")
                    } //TODO: Split between four of a kind and a full house}
                }
            }
            1 => Ranking::FiveOfAKind,
            _ => {
                println!("I was unable to determine a hand, this shouldn't happen");
                Ranking::Undetermined
            }
        };
    }
}

#[derive(Debug, Eq)]
enum Ranking {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Undetermined,
}
impl Ranking {
    fn get_value(&self) -> Option<u32> {
        match self {
            Ranking::FiveOfAKind => Some(9),
            Ranking::FourOfAKind => Some(8),
            Ranking::FullHouse => Some(7),
            Ranking::ThreeOfAKind => Some(6),
            Ranking::TwoPair => Some(5),
            Ranking::OnePair => Some(4),
            Ranking::HighCard => Some(3),
            Ranking::Undetermined => None,
        }
    }
}

impl PartialEq<Self> for Ranking {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.get_value().unwrap() > other.rank.get_value().unwrap() {
            true => Ordering::Greater,
            false => match self.rank.get_value().unwrap() < other.rank.get_value().unwrap() {
                true => Ordering::Less,
                false => Ordering::Equal,
            },
        }
    }
}
impl PartialOrd<Self> for Ranking {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.get_value().unwrap() > other.get_value().unwrap() {
            true => Some(Ordering::Greater),
            false => match self.get_value().unwrap() < other.get_value().unwrap() {
                true => Some(Ordering::Less),
                false => Some(Ordering::Equal),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_create_hands() {
        let file = File::open("input/day_7_input.txt").unwrap();
        let mut lines = BufReader::new(file).lines();
        let mut hands: Vec<Hand> = lines.into_iter().map(|l| Hand::new(&l.unwrap())).collect();
        // hands.iter().for_each(|x| {println!("{:?}", x.rank)});
        // println!("{}", Ranking::FiveOfAKind > Ranking::FourOfAKind);
        hands.sort();
        // println!("{:?}", hands);
        // hands.iter().for_each(|x| println!("Rank: {:?} - Hand: {:?}", x.rank, x.cards ));
        let mut offset = 1;
        let winnings: u64 = hands.iter().map(|(j)| {
            offset +=1;
            ((offset-1) * j.bid) as u64
        }).collect::<Vec<u64>>().iter().sum();
        println!("winnings: {}", winnings);
    }
    #[test]
    fn test_create_hands_example() {
        let file = File::open("input/day_7_example.txt").unwrap();
        let mut lines = BufReader::new(file).lines();
        let mut hands: Vec<Hand> = lines.into_iter().map(|l| Hand::new(&l.unwrap())).collect();
        // hands.iter().for_each(|x| {println!("{:?}", x.rank)});
        // println!("{}", Ranking::FiveOfAKind > Ranking::FourOfAKind);
        hands.sort();
        // println!("{:?}", hands);
        // hands.iter().for_each(|x| println!("Rank: {:?} - Hand: {:?}", x.rank, x.cards ));
        let mut offset = 1;
        let winnings: u64 = hands.iter().map(|(j)| {
            offset +=1;
            ((offset-1) * j.bid) as u64
        }).collect::<Vec<u64>>().iter().sum();
        println!("winnings: {}", winnings);
    }
}
