use itertools::Itertools;
use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");
}

fn part_one(input: HashMap<usize, usize>) -> usize {
    solve(input, 25)
}

fn part_two(input: HashMap<usize, usize>) -> usize {
    solve(input, 75)
}

fn solve(input: HashMap<usize, usize>, count: usize) -> usize {
    (0..count).fold(input, |map, _| blink(&map)).values().sum()
}

fn blink(input: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    for (key, count) in input {
        if key == &0 {
            result
                .entry(1)
                .and_modify(|x| *x += count)
                .or_insert(*count);
            continue;
        }

        let length = key.ilog10() + 1;
        if length % 2 == 0 {
            let exp = 10usize.pow(length / 2);
            let left = key / exp;
            let right = key % exp;
            result
                .entry(left)
                .and_modify(|x| *x += count)
                .or_insert(*count);
            result
                .entry(right)
                .and_modify(|x| *x += count)
                .or_insert(*count);
            continue;
        }

        result
            .entry(key * 2024)
            .and_modify(|x| *x += count)
            .or_insert(*count);
    }

    result
}

fn parse_input(input: &'static str) -> HashMap<usize, usize> {
    input
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .counts()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(part_one(parse_input(TESTDATA)), 55312);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(parse_input(DATA)), 224529);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(parse_input(DATA)), 266820198587914);
    }
}
