use std::collections::HashMap;

struct Hand {
    hand: Vec<String>,
    rank: Ranking,
    bid: u32,
}
impl Hand {
    fn new(incoming: &str) -> Self {
        let mut s = incoming.split(" ");
        let hand = s.next().unwrap().to_string();
        let bid = s.next().unwrap().parse().unwrap();
    }
    fn determine_ranking(&mut self, hand: String) {
        let mut hm = HashMap::new();
        _ = hand.chars().into_iter().map(|x| match hm.contains_key(&x) {
            true => *hm.get_mut(&x).unwrap() += 1,
            false => {
                hm.insert(x, 1);
            }
        });
        // The logic here says that based on the number of entries in the hashmap we can
        // determine which type of hand we're dealing with
        match hm.len() {
            5 => {self.rank = Ranking::HighCard}
            4 => {self.rank = Ranking::OnePair}
            3 => {
                match hm.iter().map(|(k, v)| {v}).max().unwrap() {
                    &3 => {self.rank = Ranking::ThreeOfAKind}
                    &2 => {self.rank = Ranking::TwoPair}
                    _ => {panic!("Shouldn't have more than three or less than 2")}
                }
            }
            2 => {
                    //TODO: Split between four of a kind and a full house}
                }
            1 => {self.rank = Ranking::FiveOfAKind}
            _ => {
                self.rank = Ranking::Undetermined;
                println!("I was unable to determine a hand, this shouldn't happen");
            }
        }
    }
}
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
