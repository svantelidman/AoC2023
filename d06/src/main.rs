#[derive(Debug)]
struct Struct {
}

fn main() {
    println!("Answer part 1: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2: {}", part_2(include_str!("../input.txt")));
}

fn calc_n_winners(race_time: usize, record_distance: usize) -> usize {
    (1..=race_time).filter(|press_time| (race_time - press_time) * press_time > record_distance).count()
}

fn part_1(input: &str) -> usize {
    let (times, distances) = parse_input(input);
    let legs: Vec<_> = times.into_iter().zip(distances).collect();
    legs.into_iter().map(|(race_time, record_distance)| calc_n_winners(race_time, record_distance)).product()
}

fn part_2(input: &str) -> usize {
    let (times, distances) = parse_input(input);
    let race_time = times.iter().map(|t| format!("{t}")).collect::<String>().parse::<usize>().unwrap();
    let record_distance = distances.iter().map(|d| format!("{d}")).collect::<String>().parse::<usize>().unwrap();
    calc_n_winners(race_time, record_distance)
}

fn parse_input(input: &str) ->(Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|w| w.parse::<usize>().unwrap()).collect();
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|w| w.parse::<usize>().unwrap()).collect();
    (times, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 288)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 71503)
    }
}