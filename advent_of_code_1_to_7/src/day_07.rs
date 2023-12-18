use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

#[test]
fn solve() {
    let data = std::fs::read_to_string("day_07.txt").unwrap();
    let hands: u32 = data
        .lines()
        .map(|line| line.split(" ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(cards, bid)| (cards.chars().collect_vec(), bid.parse::<u32>().unwrap()))
        .map(Hand::try_from)
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.unwrap().bid)
        .sum();
    assert_eq!(1, hands);
}

static RANKS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483

static TEST_INPUT: [([char; 5], u32); 5] = [
    (['3', '2', 'T', '3', 'K'], 765),
    (['T', '5', '5', 'J', '5'], 684),
    (['K', 'K', '6', '7', '7'], 28),
    (['K', 'T', 'J', 'J', 'T'], 220),
    (['Q', 'Q', 'Q', 'J', 'A'], 483),
];
const TEST_EXP: u32 = 5905;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: [char; 5],
    bid: u32,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(type_order) = self.hand_type.partial_cmp(&other.hand_type) {
            if type_order == Ordering::Equal {
                for (lhs, rhs) in self.cards.iter().zip(other.cards) {
                    let lhs: usize = RANKS.iter().position(|x| x == lhs).expect("bad rank");
                    let rhs: usize = RANKS.iter().position(|x| *x == rhs).expect("bad rank");
                    if let Some(o) = lhs.partial_cmp(&rhs) {
                        if o != Ordering::Equal {
                            return Some(o);
                        }
                    }
                }
                Some(Ordering::Equal)
            } else {
                Some(type_order)
            }
        } else {
            None
        }
    }
}

#[test]
fn test_ordering() {
    let actual: u32 = TEST_INPUT
        .into_iter()
        .map(Hand::from)
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();
    assert_eq!(actual, TEST_EXP);
}

impl From<([char; 5], u32)> for Hand {
    fn from((cards, bid): ([char; 5], u32)) -> Self {
        Hand {
            cards,
            bid,
            hand_type: get_hand_type(&cards).unwrap(),
        }
    }
}

impl TryFrom<(Vec<char>, u32)> for Hand {
    type Error = ();

    fn try_from((cards, bid): (Vec<char>, u32)) -> Result<Self, Self::Error> {
        if cards.len() == 5 {
            if let Ok(cards) = cards.try_into() {
                Ok(Hand {
                    cards,
                    bid,
                    hand_type: get_hand_type(&cards).unwrap(),
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,     // 1 1 1 1 1
    OnePair,      // 2 1 1 1
    TwoPairs,     // 2 2 1
    ThreeOfAKind, // 3 1 1
    FullHouse,    // 3 2
    FourOfAKind,  // 4 1
    FiveOfAKind,  // 5
}

fn get_hand_type(hand: &[char]) -> Result<HandType, ()> {
    if hand.len() != 5 {
        return Err(());
    }
    let mut check_map = HashMap::<char, u8>::new();
    for card in hand.iter() {
        let n = check_map.entry(*card).or_insert(0);
        *n += 1;
    }
    Ok(process_counts(check_map))
}

#[test]
fn test_get_hand_type() {
    let exp = [
        HandType::OnePair,
        HandType::ThreeOfAKind,
        HandType::TwoPairs,
        HandType::TwoPairs,
        HandType::ThreeOfAKind,
    ];
    for ((hand, _), exp) in TEST_INPUT.iter().zip(exp) {
        assert_eq!(get_hand_type(&hand.as_slice()), Ok(exp));
    }
}

fn process_counts(check_map: HashMap<char, u8>) -> HandType {
    let counts = check_map.values().collect::<Vec<_>>();
    let num_jokers = check_map.get(&'J').unwrap_or(&0u8);
    match counts.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if num_jokers > &0 {
                HandType::FiveOfAKind
            } else {
                match counts.contains(&&3) {
                    true => HandType::FullHouse,
                    false => HandType::FourOfAKind,
                }
            }
        }
        3 => match counts.contains(&&3) {
            true => {
                if num_jokers > &0 {
                    HandType::FourOfAKind
                } else {
                    HandType::ThreeOfAKind
                }
            }
            false => match num_jokers {
                2 => HandType::FourOfAKind,
                3 | 1 => HandType::FullHouse,
                0 => HandType::TwoPairs,
                _ => unreachable!(),
            },
        },
        4 => match num_jokers > &0 {
            true => HandType::ThreeOfAKind,
            false => HandType::OnePair,
        },
        5 => match num_jokers > &0 {
            true => HandType::OnePair,
            false => HandType::HighCard,
        },
        _ => unreachable!(),
    }
}

// 5 => HandType::HighCard,
// 4 => HandType::OnePair,
// 3 if counts.contains(&2) => HandType::TwoPairs,
// 3 if counts.contains(&3) => HandType::ThreeOfAKind,
// 2 if counts.contains(&3) => HandType::FullHouse,
// 2 if counts.contains(&4) => HandType::FourOfAKind,
// 1 => HandType::FiveOfAKind,
// _ => unreachable!(),

#[test]
fn test_process_counts() {
    let tests = [
        (
            vec![('5', 1), ('6', 1), ('7', 1), ('8', 1), ('9', 1)],
            HandType::HighCard,
        ),
        (
            vec![('5', 2), ('6', 1), ('7', 1), ('8', 1)],
            HandType::OnePair,
        ),
        (vec![('5', 2), ('6', 2), ('7', 1)], HandType::TwoPairs),
        (vec![('5', 3), ('6', 1), ('7', 1)], HandType::ThreeOfAKind),
        (vec![('5', 3), ('6', 2)], HandType::FullHouse),
        (vec![('5', 4), ('6', 1)], HandType::FourOfAKind),
        (vec![('5', 5)], HandType::FiveOfAKind),
        (
            vec![('5', 1), ('6', 1), ('7', 1), ('J', 1), ('9', 1)],
            HandType::OnePair,
        ),
        (
            vec![('5', 2), ('6', 1), ('J', 1), ('8', 1)],
            HandType::ThreeOfAKind,
        ),
        (vec![('5', 2), ('6', 2), ('J', 1)], HandType::FullHouse),
        (vec![('5', 2), ('J', 2), ('7', 1)], HandType::FourOfAKind),
        (vec![('5', 3), ('6', 1), ('J', 1)], HandType::FourOfAKind),
        (vec![('5', 3), ('J', 2)], HandType::FiveOfAKind),
        (vec![('5', 4), ('J', 1)], HandType::FiveOfAKind),
    ];
    for (counts_vec, exp) in tests.into_iter() {
        let counts = HashMap::from_iter(counts_vec);
        assert_eq!(exp, process_counts(counts));
    }
}
