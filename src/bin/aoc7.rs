use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error, Ok};

fn get_inputs() -> &'static str {
    return "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}

trait Compare {
    fn is_stronger_than(self: &Self, opponent: &Self) -> bool;
}

#[derive(Debug, Clone)]
struct Card {
    label: String,
    value: u32,
}

impl From<char> for Card {
    fn from(s: char) -> Self {
        match s.to_digit(10) {
            Some(v) => Card {
                label: s.to_string(),
                value: v,
            },
            None => {
                let value = match s {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 1,
                    'T' => 10,
                    _ => 15,
                };
                return Card {
                    label: s.to_string(),
                    value,
                };
            }
        }
    }
}

impl Compare for Card {
    fn is_stronger_than(self: &Self, opponent: &Self) -> bool {
        self.value > opponent.value
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut joker_count: usize = 0;
        let mut count_map: HashMap<&String, usize> = HashMap::new();
        for card in cards {
            if count_map.get(&card.label).is_some() {
                *count_map.get_mut(&card.label).unwrap() += 1;
            } else {
                count_map.insert(&card.label, 1);
            }
            if card.label == "J" {
                joker_count += 1;
            }
        }
        let mut counts = count_map
            .iter()
            .map(|(_, count)| *count)
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| b.partial_cmp(a).unwrap());
        if counts[0] != joker_count {
            counts[0] += joker_count;
        } else if counts.len() > 1 {
            counts[1] += joker_count;
            counts.swap(0, 1);
        }
        if counts[0] == 5 {
            return HandType::FiveOfAKind;
        }
        if counts[0] == 4 {
            return HandType::FourOfAKind;
        }
        if counts[0] == 3 {
            if counts[1] == 2 && joker_count != 2 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        if counts[0] == 2 {
            if counts[1] == 2 {
                return HandType::TwoPair;
            }
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }
}

impl Compare for HandType {
    fn is_stronger_than(self: &Self, opponent: &Self) -> bool {
        self < opponent
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl Compare for Hand {
    fn is_stronger_than(self: &Self, opponent: &Hand) -> bool {
        if self.hand_type.is_stronger_than(&opponent.hand_type) {
            return true;
        }
        if self.hand_type == opponent.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i].is_stronger_than(&opponent.cards[i]) {
                    return true;
                };
                if opponent.cards[i].is_stronger_than(&self.cards[i]) {
                    return false;
                };
            }
            return false;
        }
        return false;
    }
}

impl FromStr for Hand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = match s.split_once(" ") {
            Some(v) => {
                let card_vec = v.0.chars().collect::<Vec<_>>();
                let card_arr: [Card; 5] = [
                    Card::from(card_vec[0]),
                    Card::from(card_vec[1]),
                    Card::from(card_vec[2]),
                    Card::from(card_vec[3]),
                    Card::from(card_vec[4]),
                ];
                (card_arr, v.1.parse::<usize>().unwrap())
            }
            None => return Err(anyhow!("invalid bid input")),
        };
        let hand_type = HandType::from(&cards);
        return Ok(Hand {
            cards,
            bid,
            hand_type,
        });
    }
}

fn walk(hands: &mut Vec<Hand>, low: usize, high: usize) -> &mut Vec<Hand> {
    if low >= high {
        return hands;
    }
    let pivot: &Hand = &(hands.clone())[high];
    let mut swapped = false;
    let mut new_pivot_index = low;
    if new_pivot_index != 0 {
        new_pivot_index -= 1;
    }
    for i in low..high {
        if !hands[i].is_stronger_than(pivot) {
            if (new_pivot_index != 0 || swapped) && new_pivot_index < hands.len() - 1 {
                new_pivot_index += 1;
            }
            swapped = true;
            (*hands).swap(i, new_pivot_index);
        }
        // println!("pivot {:?} is smaller than {:?}", pivot, hands[i]);
    }
    if (new_pivot_index != 0 || swapped) && new_pivot_index < hands.len() - 1 {
        new_pivot_index += 1;
    }

    (*hands).swap(high, new_pivot_index);

    let mut new_hands = walk(hands, new_pivot_index + 1, high);

    let new_left_pivot = new_pivot_index;
    if new_left_pivot > 0 {
        new_hands = walk(new_hands, low, new_pivot_index - 1);
    }

    return new_hands;
}

fn sort_hands(hands: &mut Vec<Hand>) -> &mut Vec<Hand> {
    return walk(hands, 0, hands.len() - 1);
}

fn main() {
    let mut hands = get_inputs()
        .lines()
        .flat_map(|line| Hand::from_str(line))
        .collect::<Vec<_>>();
    let sorted = sort_hands(&mut hands);
    // for hand in sorted {
    //     println!("{:?}", hand);
    // }
    let count = sorted
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            return (i + 1) * hand.bid;
        })
        .fold(0, |sum, count| sum + count);
    println!("count: {:?}", count);
}
