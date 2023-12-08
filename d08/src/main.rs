use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"(...) = \((...), (...)\)").unwrap();
        let caps = re.captures(s).unwrap();
        let name = String::from(caps.get(1).unwrap().as_str());
        let left = String::from(caps.get(2).unwrap().as_str());
        let right = String::from(caps.get(3).unwrap().as_str());
        Self {name, left, right}
    }
}

#[derive(Debug)]
struct Directions {
    directions: Vec<char>,
    next_char_ind: usize
}

impl Directions {
    fn new(directions: &str) -> Self {
        Self {
            directions: directions.chars().collect(),
            next_char_ind: 0
        }
    }

    fn reset(&mut self) {
        self.next_char_ind = 0;
    }

    fn next(&mut self) -> char {
        let c = self.directions[self.next_char_ind].clone();
        self.next_char_ind += 1;
        if self.next_char_ind == self.directions.len() {
            self.next_char_ind = 0;
        }
        c
    }
}

fn main() {
    println!("Answer part 1: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2: {}", part_2(include_str!("../input.txt")));
}

fn parse_input(input: &str) -> (Directions, BTreeMap<String, Node>) {
    let mut it = input.lines();
    let directions = Directions::new(it.next().unwrap());
    let it = it.skip(1);
    let nodes = it
        .map(|s| {
            let node = Node::new(s);
            let name = node.name.clone();
            (name, node)
        }
        ).collect();
    (directions, nodes)
}

fn part_1(input: &str) -> usize {
    let (mut directions, nodes) = parse_input(input);
    let mut current_node = String::from("AAA");
    let mut n_steps = 0;
    while current_node != "ZZZ" {
        n_steps += 1;
        current_node = next_node(&current_node, &mut directions, &nodes);
    }
    n_steps
}

fn next_node(current_node: &String, directions: &mut Directions, nodes: &BTreeMap<String, Node>) -> String {
    if directions.next() == 'L' {
        nodes.get(current_node).unwrap().left.clone()
    } else {
        nodes.get(current_node).unwrap().right.clone()
    }
}

fn part_2(input: &str) -> usize {
    let (mut directions, nodes) = parse_input(input);
    let start_nodes: Vec<_> = nodes.keys().filter(|n| n.ends_with('A')).map(|n| n.clone()).collect();
    let loop_lengths: Vec<_> = start_nodes.iter()
        .map(|start_node| {
            let mut n_steps: usize = 0;
            let mut current_node = start_node.clone();
            directions.reset();
            while !current_node.ends_with('Z') {
                current_node = next_node(&current_node, &mut directions, &nodes);
                n_steps += 1;
            }
            n_steps
        }
    ).collect();

    let largest= loop_lengths.iter().max().unwrap();
    let mut answer = largest.clone();
    while !loop_lengths.iter().all(|o| answer % o == 0) {
        answer += largest
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part_1() {
        assert_eq!(part_1(include_str!("../test1.txt")), 2)
    }

    #[test]
    fn test2_part_1() {
        assert_eq!(part_1(include_str!("../test2.txt")), 6)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test3.txt")), 6)
    }
}