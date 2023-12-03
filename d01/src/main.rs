fn main() {
    println!("Sum of codes part 1: {}", part_1(include_str!("../input.txt")));
    println!("Sum of codes part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    parse_codes(input).into_iter().sum::<usize>()
}

fn part_2(input: &str) -> usize {
    parse_codes_2(input).into_iter().sum::<usize>()
}

fn parse_codes(input: &str) -> Vec<usize> {
    let mut codes: Vec<usize> = vec!();
    for line in input.lines() {
        let first_digit = line.chars().into_iter().filter(|c| c.is_digit(10)).next().unwrap();
        let last_digit = line.chars().rev().into_iter().filter(|c| c.is_digit(10)).next().unwrap();
        let str = format!("{first_digit}{last_digit}");
        let code = str.parse::<usize>().unwrap();
        codes.push(code);
    }
    codes
}

fn scan_for_digit(line: &str, indexes: Vec<usize>) -> usize {
    let str_digits = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
    for ind in indexes {
        let c = line[ind..].chars().next().unwrap();
        if c.is_digit(10) {
            return c.to_digit(10).unwrap() as usize;
        } else {
            if let Some(pos) = str_digits.iter().position(|sd| line[ind..].starts_with(sd)) {
                return pos + 1;
            }
        }
    }
    0
}

fn parse_codes_2(input: &str) -> Vec<usize> {
    let mut codes: Vec<usize> = vec!();
    for line in input.lines() {
        let last_ind = line.len() - 1;
        let first_digit = scan_for_digit(line, (0..=last_ind).collect());
        let last_digit = scan_for_digit(line, (0..=last_ind).rev().collect());
        let code = last_digit + 10 * first_digit;
        codes.push(code);
    }
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 142)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test2.txt")), 281)
    }
}