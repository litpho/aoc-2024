use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, line_ending, one_of},
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};
use std::{collections::hash_map::Values, collections::HashMap};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(grid: &Grid) -> usize {
    grid.antennae()
        .flat_map(|antennae| antinodes(antennae, grid.bounds(), false))
        .unique()
        .count()
}

fn part_two(grid: &Grid) -> usize {
    grid.antennae()
        .flat_map(|antennae| antinodes(antennae, grid.bounds(), true))
        .unique()
        .count()
}

fn antinodes(
    antennae: &[(usize, usize)],
    bounds: &(usize, usize),
    part2: bool,
) -> Vec<(usize, usize)> {
    antennae
        .iter()
        .permutations(2)
        .flat_map(|pair| create_pairs(pair, bounds, part2))
        .collect::<Vec<(usize, usize)>>()
}

fn create_pairs(
    orig: Vec<&(usize, usize)>,
    bounds: &(usize, usize),
    part2: bool,
) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let (x1, y1) = *orig[0];
    let (x2, y2) = *orig[1];
    let x_diff = (x1 as isize) - (x2 as isize);
    let y_diff = (y1 as isize) - (y2 as isize);

    if part2 {
        let mut count = 0;
        while let Some(pair) = check_pair(
            x1 as isize + x_diff * count,
            y1 as isize + y_diff * count,
            bounds,
        ) {
            vec.push(pair);
            count += 1;
        }
    } else if let Some(pair) = check_pair(x1 as isize + x_diff, y1 as isize + y_diff, bounds) {
        vec.push(pair);
    }

    vec
}

fn check_pair(x: isize, y: isize, bounds: &(usize, usize)) -> Option<(usize, usize)> {
    if x >= 0 && x < bounds.0 as isize && y >= 0 && y < bounds.1 as isize {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

struct Grid {
    bounds: (usize, usize),
    map: HashMap<char, Vec<(usize, usize)>>,
}

impl Grid {
    pub fn new(input: Vec<Vec<Option<char>>>) -> Self {
        let bounds = (input[0].len(), input.len());
        let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(c) = cell {
                    map.entry(*c).or_default().push((x, y));
                }
            }
        }

        Self { bounds, map }
    }

    pub fn antennae(&self) -> Values<char, Vec<(usize, usize)>> {
        self.map.values()
    }

    pub fn bounds(&self) -> &(usize, usize) {
        &self.bounds
    }
}

fn parse_input(input: &'static str) -> Result<Grid> {
    let (_, input) = parse(input)?;

    let grid = Grid::new(input);

    Ok(grid)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many1(alt((parse_dot, parse_char)))(input)
}

fn parse_char(input: &str) -> IResult<&str, Option<char>> {
    map(
        one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
        Some,
    )(input)
}

fn parse_dot(input: &str) -> IResult<&str, Option<char>> {
    value(None, complete::char('.'))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(14, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(318, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(34, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(1126, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
