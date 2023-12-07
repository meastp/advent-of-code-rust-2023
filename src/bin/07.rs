advent_of_code::solution!(7);

use std::cmp::Ordering;
use std::fmt;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    value: u32,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.value {
                14 => 'A',
                13 => 'K',
                12 => 'Q',
                11 => 'J',
                10 => 'T',
                _ => std::char::from_digit(self.value, 10).unwrap(),
            }
        )
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    hand: Vec<Card>,
    bet: u32,
}

impl Hand {
    fn get_type(&self) -> u8 {
        let iter = self.hand.clone().into_iter();

        let counts = iter.counts();

        let counts_vec = counts
            .values()
            .cloned()
            .map(|v| u8::try_from(v).unwrap())
            .collect_vec();

        // alle kortene er ulike
        if counts_vec.len() == 5 {
            1
        }
        // ett par
        else if counts_vec.len() == 4 {
            2
        }
        // to par
        else if counts_vec.len() == 3 && counts_vec.iter().filter(|v| *v == &2u8).count() == 2 {
            3
        }
        // tre like
        else if counts_vec.len() == 3
            && counts_vec.iter().any(|v| *v == 3u8) == true
            && counts_vec.iter().any(|v| *v == 2u8) == false
        {
            4
        }
        // hus
        else if counts_vec.len() == 2
            && counts_vec.iter().any(|v| *v == 3u8) == true
            && counts_vec.iter().any(|v| *v == 2u8) == true
        {
            5
        }
        // fire like
        else if counts_vec.len() == 2 && counts_vec.iter().any(|v| *v == 4u8) == true {
            6
        }
        // fem like
        else if counts_vec.len() == 1 {
            7
        } else {
            panic!("ukjent hånd: {:?} {:?}", self.hand, counts_vec);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // self.hand.partial_cmp(&other.hand)

        let self_type = self.get_type();
        let other_type = other.get_type();

        let result = self_type.partial_cmp(&other_type);

        if result.unwrap() == Ordering::Equal {
            self.hand.partial_cmp(&other.hand)
        } else {
            result
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let mut hand = Hand {
            hand: Vec::new(),
            bet: 0,
        };

        if let Some(hand_str) = iter.next() {
            for c in hand_str.chars() {
                const RADIX: u32 = 10;
                hand.hand.push(match c {
                    'A' => Card { value: 14 },
                    'K' => Card { value: 13 },
                    'Q' => Card { value: 12 },
                    'J' => Card { value: 11 },
                    'T' => Card { value: 10 },
                    _ => Card {
                        value: c.to_digit(RADIX).unwrap(),
                    },
                });
            }
        }
        if let Some(bet_str) = iter.next() {
            hand.bet = bet_str.parse().unwrap();
        }

        hands.push(hand);
    }

    hands.sort();

    //println!("{:?}", hands);

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.bet * (u32::try_from(idx).unwrap() + 1))
            .sum(),
    )
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card2 {
    value: u32,
}

impl fmt::Debug for Card2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.value {
                13 => 'A',
                12 => 'K',
                11 => 'Q',
                10 => 'T',
                1 => 'J',
                _ => std::char::from_digit(self.value, 10).unwrap(),
            }
        )
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand2 {
    hand: Vec<Card2>,
    bet: u32,
}

impl Hand2 {
    fn get_type(&self) -> u8 {
        let iter = self.hand.iter();

        let counts = iter.counts();

        let mut counts_vec = counts
            .values()
            .cloned()
            .map(|v| u8::try_from(v).unwrap())
            .collect_vec();

        counts_vec.sort();

        let joker_card = Card2 { value: 1 };
        let counts_joker = counts.get(&joker_card);

        match counts_vec[..] {
            [1u8, 1u8, 1u8, 1u8, 1u8] if counts_joker == None => 1u8,
            [1u8, 1u8, 1u8, 1u8, 1u8] if counts_joker == Some(&1usize) => 2u8,

            [1u8, 1u8, 1u8, 2u8] if counts_joker == None => 2u8,
            [1u8, 1u8, 1u8, 2u8] if counts_joker == Some(&1usize) => 4u8,
            [1u8, 1u8, 1u8, 2u8] if counts_joker == Some(&2usize) => 4u8,

            [1u8, 1u8, 3u8] if counts_joker == None => 4u8,
            [1u8, 1u8, 3u8] if counts_joker == Some(&1usize) => 6u8,
            [1u8, 1u8, 3u8] if counts_joker == Some(&3usize) => 6u8,

            [1u8, 2u8, 2u8] if counts_joker == None => 3u8,
            [1u8, 2u8, 2u8] if counts_joker == Some(&1usize) => 5u8,
            [1u8, 2u8, 2u8] if counts_joker == Some(&2usize) => 6u8,

            [1u8, 4u8] if counts_joker == None => 6u8,
            [1u8, 4u8] if counts_joker == Some(&1usize) => 7u8,
            [1u8, 4u8] if counts_joker == Some(&4usize) => 7u8,

            [2u8, 3u8] if counts_joker == None => 5u8,
            [2u8, 3u8] if counts_joker == Some(&2usize) => 7u8,
            [2u8, 3u8] if counts_joker == Some(&3usize) => 7u8,

            [5u8] => 7u8,

            _ => panic!(
                "ukjent hånd: {:?} {:?} {:?} {:?}",
                self.hand, counts, counts_vec, counts_joker
            ),
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // self.hand.partial_cmp(&other.hand)

        let self_type = self.get_type();
        let other_type = other.get_type();

        let result = self_type.partial_cmp(&other_type);

        if result.unwrap() == Ordering::Equal {
            self.hand.partial_cmp(&other.hand)
        } else {
            result
        }
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        let mut hand = Hand2 {
            hand: Vec::new(),
            bet: 0,
        };

        if let Some(hand_str) = iter.next() {
            for c in hand_str.chars() {
                const RADIX: u32 = 10;
                hand.hand.push(match c {
                    'A' => Card2 { value: 13 },
                    'K' => Card2 { value: 12 },
                    'Q' => Card2 { value: 11 },
                    'T' => Card2 { value: 10 },
                    'J' => Card2 { value: 1 },
                    _ => Card2 {
                        value: c.to_digit(RADIX).unwrap(),
                    },
                });
            }
        }
        if let Some(bet_str) = iter.next() {
            hand.bet = bet_str.parse().unwrap();
        }

        hands.push(hand);
    }

    hands.sort();

    //println!("{:?}", hands);

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.bet * (u32::try_from(idx).unwrap() + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
