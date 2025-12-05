use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one (2): {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");
}

fn part_one(input: &HashMap<(isize, isize), Letter>) -> usize {
    input
        .iter()
        .filter(|(_, letter)| letter == &&Letter::X)
        .map(|((x, y), _)| solve_xmas(input, *x, *y))
        .sum()
}

const ONE_DIRECTIONS: [(isize, isize); 8] = [
    // up
    (0, -1),
    // up right
    (1, -1),
    // right
    (1, 0),
    // down right
    (1, 1),
    // down
    (0, 1),
    // down left
    (-1, 1),
    // left
    (-1, 0),
    // up left
    (-1, -1),
];

fn solve_xmas(input: &HashMap<(isize, isize), Letter>, x: isize, y: isize) -> usize {
    ONE_DIRECTIONS
        .iter()
        .map(|matrix| solve_xmas_dir(input, x, y, matrix))
        .sum()
}

fn solve_xmas_dir(
    input: &HashMap<(isize, isize), Letter>,
    x: isize,
    y: isize,
    matrix: &(isize, isize),
) -> usize {
    if let Some(Letter::M) = input.get(&(x + matrix.0, y + matrix.1)) {
        if let Some(Letter::A) = input.get(&(x + matrix.0 * 2, y + matrix.1 * 2)) {
            if let Some(Letter::S) = input.get(&(x + matrix.0 * 3, y + matrix.1 * 3)) {
                return 1;
            }
        }
    }

    0
}

fn part_two(input: &HashMap<(isize, isize), Letter>) -> usize {
    input
        .iter()
        .filter(|(_, letter)| letter == &&Letter::A)
        .filter(|((x, y), _)| solve_mas(input, *x, *y))
        .count()
}

const TWO_DIRECTIONS: [(isize, isize); 4] = [
    // up right
    (1, -1),
    // down right
    (1, 1),
    // down left
    (-1, 1),
    // up left
    (-1, -1),
];

fn solve_mas(input: &HashMap<(isize, isize), Letter>, x: isize, y: isize) -> bool {
    let result: usize = TWO_DIRECTIONS
        .iter()
        .map(|matrix| solve_mas_dir(input, x, y, matrix))
        .sum();

    result > 1
}

fn solve_mas_dir(
    input: &HashMap<(isize, isize), Letter>,
    x: isize,
    y: isize,
    matrix: &(isize, isize),
) -> usize {
    if let Some(Letter::M) = input.get(&(x + matrix.0, y + matrix.1)) {
        if let Some(Letter::S) = input.get(&(x - matrix.0, y - matrix.1)) {
            return 1;
        }
    }

    0
}

#[derive(Debug, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl From<char> for Letter {
    fn from(c: char) -> Self {
        match c {
            'X' => Letter::X,
            'M' => Letter::M,
            'A' => Letter::A,
            'S' => Letter::S,
            _ => panic!("Unknown letter: {c}"),
        }
    }
}

fn parse(input: &str) -> HashMap<(isize, isize), Letter> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), Letter::from(c)))
        })
        .collect::<HashMap<(isize, isize), Letter>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(part_one(&parse(TESTDATA)), 18);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(DATA)), 2573);
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(part_two(&parse(TESTDATA)), 9);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(DATA)), 1850);
    }
}
