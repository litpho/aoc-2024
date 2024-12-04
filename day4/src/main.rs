use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
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

fn solve_xmas(input: &HashMap<(isize, isize), Letter>, x: isize, y: isize) -> usize {
    let mut result = 0;

    // top
    result += solve_xmas_dir(input, x, y, &[(0, -1), (0, -2), (0, -3)]);
    // top right
    result += solve_xmas_dir(input, x, y, &[(1, -1), (2, -2), (3, -3)]);
    // right
    result += solve_xmas_dir(input, x, y, &[(1, 0), (2, 0), (3, 0)]);
    // down right
    result += solve_xmas_dir(input, x, y, &[(1, 1), (2, 2), (3, 3)]);
    // down
    result += solve_xmas_dir(input, x, y, &[(0, 1), (0, 2), (0, 3)]);
    // down left
    result += solve_xmas_dir(input, x, y, &[(-1, 1), (-2, 2), (-3, 3)]);
    // left
    result += solve_xmas_dir(input, x, y, &[(-1, 0), (-2, 0), (-3, 0)]);
    // top left
    result += solve_xmas_dir(input, x, y, &[(-1, -1), (-2, -2), (-3, -3)]);

    result
}

fn solve_xmas_dir(
    input: &HashMap<(isize, isize), Letter>,
    x: isize,
    y: isize,
    matrix: &[(isize, isize); 3],
) -> usize {
    if let Some(Letter::M) = input.get(&(x + matrix[0].0, y + matrix[0].1)) {
        if let Some(Letter::A) = input.get(&(x + matrix[1].0, y + matrix[1].1)) {
            if let Some(Letter::S) = input.get(&(x + matrix[2].0, y + matrix[2].1)) {
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

fn solve_mas(input: &HashMap<(isize, isize), Letter>, x: isize, y: isize) -> bool {
    let mut result = 0;

    // top right
    result += solve_mas_dir(input, x, y, (1, -1));
    // down right
    result += solve_mas_dir(input, x, y, (1, 1));
    // down left
    result += solve_mas_dir(input, x, y, (-1, 1));
    // top left
    result += solve_mas_dir(input, x, y, (-1, -1));

    result > 1
}

fn solve_mas_dir(
    input: &HashMap<(isize, isize), Letter>,
    x: isize,
    y: isize,
    matrix: (isize, isize),
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
        assert_eq!(18, part_one(&parse(TESTDATA)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2573, part_one(&parse(DATA)));
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(9, part_two(&parse(TESTDATA)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1850, part_two(&parse(DATA)));
    }
}
