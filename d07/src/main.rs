use std::collections::BTreeMap;

#[derive(Debug)]
struct Hand {
    bid: usize,
    _cards: Vec<usize>,
    _hand_type: usize,
    sort_key: (usize, Vec<usize>)
}

impl Hand { 
    fn determine_card_counts(cards: &Vec<usize>) -> Vec<usize> {
        let mut card_counts:  BTreeMap<usize, usize> = BTreeMap::new();
        cards.iter()
            .for_each(|c| 
                if let Some(prev_count) = card_counts.get_mut(c) {
                    *prev_count += 1;
                } else {
                    card_counts.insert(c.clone(), 1);
                }
            );
        let mut card_counts: Vec<usize> = card_counts.values().map(|v| v.clone()).collect();
        card_counts.sort();
        card_counts.reverse();
        card_counts
    }

    fn new_1(cards: Vec<usize>, bid: usize) -> Self {
        fn determine_hand_type(card_counts: Vec<usize>) -> usize {
            match card_counts[0] {
                5 => 7,
                4 => 6,
                3 => if card_counts[1] == 2 {5 } else { 4 },
                2 => if card_counts[1] == 2 { 3 } else { 2 },
                _ => 1
            }
        }

        let card_counts = Hand::determine_card_counts(&cards);
        let hand_type = determine_hand_type(card_counts);
        let sort_key = (hand_type, cards.clone());
        Self {_cards: cards, bid, _hand_type: hand_type, sort_key}
    }

    fn new_2(cards: Vec<usize>, bid: usize) -> Self {
        fn determine_hand_type(card_counts: Vec<usize>, n_jokers: usize) -> usize {
            if n_jokers == 5 { return 7 }
            match card_counts[0] {
                5 => 7,
                4 => if n_jokers == 1 {7} else {6},
                3 => match n_jokers {
                    2 => 7,
                    1 => 6,
                    _ => if card_counts[1] == 2 {5 } else { 4 }
                },
                2 => match n_jokers {
                    3 => 7,
                    2 => 6,
                    1 => if card_counts[1] == 2 {5} else {4},
                    _ => if card_counts[1] == 2 { 3 } else { 2 }
                },
                1 => match n_jokers {
                    4 => 7,
                    3 => 6,
                    2 => 4,
                    1 => 2,
                    _ => 1
                },
                _ => 7
            }
        }

        let non_jokers: Vec<_> = cards.iter().filter(|c| **c != 1).map(|c| c.clone()).collect();
        let n_jokers = cards.len() - non_jokers.len();

        let card_counts = Hand::determine_card_counts(&non_jokers);
        let hand_type = determine_hand_type(card_counts, n_jokers);
        let sort_key = (hand_type, cards.clone());
        Self {_cards: cards, bid, _hand_type: hand_type, sort_key}
    }
}


fn main() {
    println!("Answer part 1: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    let mut cards = parse_input(&input, 1);
    cards.sort_by_key(|c| c.sort_key.clone());
    cards.iter().zip(1..).map(|(c, rank)| c.bid * rank).sum()
}

fn part_2(input: &str) -> usize {
    let mut cards = parse_input(&input, 2);
    cards.sort_by_key(|c| c.sort_key.clone());
    cards.iter().zip(1..).map(|(c, rank)| c.bid * rank).sum()
}

fn parse_input(input: &str, part: usize) -> Vec<Hand> {
    fn parse_hand(line: &str, part: usize) -> Hand {
        let mut it = line.split_whitespace();
        let cards = it.next().unwrap().chars()
            .map(|c| 
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => if part == 1 {11} else {1},
                    'T' => 10,
                    _ => c.to_digit(10).unwrap() as usize
                }
            ).collect();
        let bid = it.next().unwrap().parse::<usize>().unwrap();
        if part == 1 {
            Hand::new_1(cards, bid)
        } else {
            Hand::new_2(cards, bid)
        }
    }
    input.lines().map(|line| parse_hand(line, part)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 6440)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 5905)
    }
}