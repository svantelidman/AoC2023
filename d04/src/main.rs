use regex::Regex;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Card {
    _id: usize,
    draw_numbers: BTreeSet<usize>,
    ticket_numbers: BTreeSet<usize>,
}

impl Card {
    fn winning_numbers(&self) -> BTreeSet<usize> {
        self.ticket_numbers.intersection(&self.draw_numbers).map(|tn| *tn).collect()
    }

    fn get_points(&self) -> usize {
        let n_winners = self.winning_numbers().len();
        if n_winners == 0 {
            0
        } else {
            2usize.pow((n_winners - 1) as u32)
        }
    }
}

fn main() {
    println!("Answer part 1: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2: {}", part_2(include_str!("../input.txt")));
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(|line| parse_card(line)).collect()
}

fn parse_numbers(s: &str) -> BTreeSet<usize> {
    s.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect()
}

fn parse_card(line: &str) -> Card {
    let re = Regex::new(r"Card\s+(\d+):([\d\s]+)\|([\d\s]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let draw_numbers = parse_numbers(caps.get(2).unwrap().as_str());
    let ticket_numbers = parse_numbers(caps.get(3).unwrap().as_str());
    Card{_id: id, draw_numbers, ticket_numbers}
}

fn part_1(input: &str) -> usize {
    let cards = parse_cards(input);
    cards.iter().map(|card| card.get_points()).sum()
}

fn part_2(input: &str) -> usize {
    let cards = parse_cards(input);
    let mut card_counts: Vec<usize> = vec!();
    card_counts.resize(cards.len(), 1);
    for card_ind in 0..cards.len() {
        let n_winners = cards[card_ind].winning_numbers().len();
        let n_cards = card_counts[card_ind];
        for delta in 1..=n_winners {
            card_counts[card_ind + delta] += n_cards
        }
    }
    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 30)
    }
}