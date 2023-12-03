use regex::Regex;

#[derive(Debug)]
struct Draw {
    n_red: usize,
    n_green: usize,
    n_blue: usize
}

impl Draw {
    fn new(s: &str) -> Draw {
        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();
        let n_red = if re_red.is_match(s) {
            re_red.captures(s).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()
        } else {
            0
        };
        let n_green = if re_green.is_match(s) {
            re_green.captures(s).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()
        } else {
            0
        };
        let n_blue = if re_blue.is_match(s) {
            re_blue.captures(s).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()
        } else {
            0
        };
        Draw {n_red, n_green, n_blue}
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>
}

impl Game {
    fn new(line: &str) -> Game {
        let re = Regex::new(r"Game (\d+): (.+)").unwrap();
        let caps = re.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let draws_str = caps.get(2).unwrap().as_str();
        let draws: Vec<Draw> = draws_str.split("; ").map(|ds| Draw::new(ds)).collect();
        Game { id, draws}
    }
}

fn main() {
    println!("Sum of game ids part 1: {}", part_1(include_str!("../input.txt")));
    println!("Sum of powers part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    let games = parse_games(input);
    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;
    games.iter().filter(|g| g.draws.iter().all(|d| d.n_red <= max_reds && d.n_green <= max_greens && d.n_blue <= max_blues)).map(|g| g.id).sum()
}

fn part_2(input: &str) -> usize {
    let games = parse_games(input);
    games.iter().map(|g| g.draws.iter()
        .fold((0, 0, 0), |(min_red, min_green, min_blue) , d| (min_red.max(d.n_red), min_green.max(d.n_green), min_blue.max(d.n_blue)) ))
        .map(|(min_red, min_green, min_blue)| min_red * min_green * min_blue ).sum()
}


fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(|l| Game::new(l)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 8)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 2286)
    }
}