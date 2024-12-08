use anyhow::Result;
use itertools::Itertools;
use nom::character::complete::one_of;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    // let (took, result) = took::took(|| parse_input(DATA));
    // println!("Time spent parsing: {took}");
    // let input = result?;
    //
    // let (took, result) = took::took(|| part_two(&input));
    // println!("Result part two: {result}");
    // println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<Option<char>>]) -> usize {
    // println!("input: {input:?}");

    let bounds = (input[0].len(), input.len());
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .filter_map(|(x, c)| {
                if c.is_some() {
                    Some((x, c.unwrap()))
                } else {
                    None
                }
            })
            .for_each(|(x, c)| map.entry(c).or_default().push((x, y)));
    });

    let mut x = map
        .keys()
        .flat_map(|c| antinodes(c, &map, &bounds))
        .collect::<Vec<(usize, usize)>>();

    x.sort();

    println!("antinodes: {}", x.len());

    x.dedup();

    println!("antinodes: {}", x.len());

    map.keys()
        .flat_map(|c| antinodes(c, &map, &bounds))
        .unique()
        .count()
}

fn antinodes(
    key: &char,
    map: &HashMap<char, Vec<(usize, usize)>>,
    bounds: &(usize, usize),
) -> Vec<(usize, usize)> {
    map.get(key)
        .unwrap()
        .iter()
        .combinations(2)
        .flat_map(|pair| create_pairs(pair, bounds))
        .collect::<Vec<(usize, usize)>>()
}

fn create_pairs(orig: Vec<&(usize, usize)>, bounds: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    let (x1, y1) = *orig[0];
    let (x2, y2) = *orig[1];
    let x_diff = (x1 as isize) - (x2 as isize);
    let y_diff = (y1 as isize) - (y2 as isize);

    let first = (x1 as isize + x_diff, y1 as isize + y_diff);
    let second = (x2 as isize - x_diff, y2 as isize - y_diff);

    if let Some(pair) = check_pair(&first, bounds) {
        vec.push(pair);
    }
    if let Some(pair) = check_pair(&second, bounds) {
        vec.push(pair);
    }

    vec
}

fn check_pair(pair: &(isize, isize), bounds: &(usize, usize)) -> Option<(usize, usize)> {
    let (x, y) = *pair;
    if x >= 0 && x < bounds.0 as isize && y >= 0 && y < bounds.1 as isize {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn part_two(input: &[Vec<Option<char>>]) -> usize {
    unimplemented!()
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<Option<char>>>> {
    let (_, input) = parse(input)?;

    Ok(input)
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

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(124060392153684, part_two(&parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
