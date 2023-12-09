fn main() {
    println!("Answer part 1: {}", part_1(include_str!("../input.txt")));
    println!("Answer part 2: {}", part_2(include_str!("../input.txt")));
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|line| line.split_whitespace().map(|s| s.parse::<isize>().unwrap()).collect()).collect()
}

fn extrapolate_forward(sequence: &Vec<isize>) -> isize {
    //println!("{:?}", sequence);
    if sequence.iter().all(|v| *v == 0) {
        0
    } else {
        let diff_sequence = (1..sequence.len()).zip(0..sequence.len()).map(|(ind_1, ind_2)| sequence[ind_1] - sequence[ind_2]).collect();
        extrapolate_forward(&diff_sequence) + sequence.last().unwrap()
    }
}

fn part_1(input: &str) -> isize {
    let sequences = parse_input(input);
    sequences.iter().map(|sequence| extrapolate_forward(sequence)).sum()
}

fn extrapolate_backward(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|v| *v == 0) {
        0
    } else {
        let diff_sequence = (1..sequence.len()).zip(0..sequence.len()).map(|(ind_1, ind_2)| sequence[ind_1] - sequence[ind_2]).collect();
        sequence.first().unwrap() - extrapolate_backward(&diff_sequence)
    }
}

fn part_2(input: &str) -> isize {
    let sequences = parse_input(input);
    sequences.iter().map(|sequence| extrapolate_backward(sequence)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 114)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 2)
    }
}