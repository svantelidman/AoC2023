#[derive(Debug, Clone)]
struct Number {
    number: usize,
    start_col: isize,
    end_col: isize,
    line: isize,
}

fn main() {
    println!("Sum of valid part numbers: {}", part_1(include_str!("../input.txt")));
    println!("Sum of gear ratios part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    fn is_symbol_at(l: isize, c: isize, grid: &Vec<Vec<char>>) -> bool {
        if l >= 0 && l < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
            !grid[l as usize][c as usize].is_digit(10) && grid[l as usize][c as usize] != '.'
        } else {
            false
        }
    }        
    let grid = load_grid(input);
    let numbers = parse_numbers(&grid);
    let deltas = vec!((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1));
    let is_position_adjacent_to_symbol = |l: isize, c: isize| -> bool { deltas.iter().any(|(dl, dc)| is_symbol_at(l + dl, c + dc, &grid))};
    let is_number_adjacent_to_symbol = |number: &Number| -> bool {(number.start_col..=number.end_col).any(|c| is_position_adjacent_to_symbol(number.line, c))};
    numbers.iter().filter(|n| is_number_adjacent_to_symbol(n)).map(|n| n.number).sum()
}

fn part_2(input: &str) -> usize {
    let grid = load_grid(input);
    let numbers = parse_numbers(&grid);
    let asterisk_positions = parse_asterisks(&grid);
    asterisk_positions.iter()
        .map(|pos| get_adjacent_numbers(pos.0, pos.1, &numbers))
        .filter(|adjacent| adjacent.len() == 2)
        .map(|adjacent| adjacent[0].number * adjacent[1].number)
        .sum()
}

fn get_adjacent_numbers(l: isize, c: isize, numbers: &Vec<Number>) -> Vec<Number> {
    fn is_adjacent(l: isize, c: isize, n: &Number) -> bool {
        let l_min = n.line - 1;
        let l_max = n.line + 1;
        let c_min = n.start_col - 1;
        let c_max = n.end_col + 1;
        l >= l_min && l <= l_max && c >= c_min && c <= c_max        
    }
    numbers.iter().filter(|n| is_adjacent(l, c, n)).map(|n| n.clone()).collect()
}

fn parse_asterisks(grid: &Vec<Vec<char>>) -> Vec<(isize, isize)> {
    let n_lines = grid.len() as usize;
    let n_cols = grid[0].len() as usize;
    let mut positions = vec!();
    for l in 0..n_lines {
        for c in 0..n_cols {
            if grid[l][c] == '*' {
                positions.push((l as isize, c as isize))
            }
        }
    }
    positions
}

fn parse_numbers(grid: &Vec<Vec<char>>) -> Vec<Number> {
    let n_lines = grid.len() as isize;
    let n_cols = grid[0].len() as isize;
    let mut numbers: Vec<Number> = vec!();
    for l in 0..n_lines {
        let mut c: isize = 0;
        while c < n_cols {
            if grid[l as usize][c as usize].is_digit(10) {
                let mut number = grid[l as usize][c as usize].to_digit(10).unwrap();
                let start_col: isize = c;
                loop {
                    c += 1;
                    if c < n_cols && grid[l as usize][c as usize].is_digit(10) {
                        number = number * 10 + grid[l as usize][c as usize].to_digit(10).unwrap();
                    } else {
                        break
                    }
                }
                numbers.push(Number{number: number as usize, start_col, end_col: c - 1, line: l});
            } else {
                c += 1;
            }
        }
    }
    numbers
}


fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 4361)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 467835)
    }
}