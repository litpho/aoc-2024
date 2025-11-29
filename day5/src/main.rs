use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

type Rules = HashMap<usize, Vec<usize>>;
type Pages = Vec<Vec<usize>>;

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let (rules, pages) = result?;

    let (took, result) = took::took(|| part_one(&rules, &pages));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let (rules, pages) = result?;

    let (took, result) = took::took(|| part_two(&rules, pages));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(rules: &Rules, pages: &Pages) -> usize {
    pages
        .iter()
        .filter_map(|pages| is_valid(rules, pages, true))
        .map(|v| middle(v))
        .sum()
}

fn part_two(rules: &Rules, pages: Pages) -> usize {
    let invalid = pages
        .iter()
        .filter_map(|pages| is_valid(rules, pages, false))
        .cloned()
        .collect::<Vec<Vec<usize>>>();

    invalid
        .into_iter()
        .map(|mut pages| {
            pages.sort_by(comparator(rules));
            pages
        })
        .map(|v| middle(&v))
        .sum()
}

fn is_valid<'a>(input: &Rules, pages: &'a Vec<usize>, valid: bool) -> Option<&'a Vec<usize>> {
    for (i, page) in pages.iter().enumerate() {
        if let Some(rules) = input.get(page) {
            if pages[..i].iter().any(|b| rules.contains(b)) {
                return if valid { None } else { Some(pages) };
            }
        }
    }
    if valid {
        Some(pages)
    } else {
        None
    }
}

fn middle(v: &[usize]) -> usize {
    *v.get(v.len() / 2).unwrap()
}

fn comparator(input: &Rules) -> impl Fn(&usize, &usize) -> std::cmp::Ordering + use<'_> {
    |a, b| {
        if input.get(a).is_some_and(|p| p.contains(b)) {
            return std::cmp::Ordering::Less;
        }
        if input.get(b).is_some_and(|p| p.contains(a)) {
            return std::cmp::Ordering::Greater;
        }

        std::cmp::Ordering::Equal
    }
}

fn parse_input(input: &'static str) -> Result<(Rules, Pages)> {
    let (_, input) = parse(input)?;

    Ok(input)
}

fn parse(input: &str) -> IResult<&str, (Rules, Pages)> {
    separated_pair(parse_rules, pair(line_ending, line_ending), parse_pages)(input)
}

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    map(separated_list1(line_ending, parse_rule), |v| {
        let mut map = Rules::new();
        for (a, b) in v {
            map.entry(a).or_default().push(b);
        }
        map
    })(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        separated_pair(complete::u32, tag("|"), complete::u32),
        |(a, b)| (a as usize, b as usize),
    )(input)
}

fn parse_pages(input: &str) -> IResult<&str, Pages> {
    separated_list1(line_ending, parse_page)(input)
}

fn parse_page(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map(complete::u32, |a| a as usize))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let (rules, pages) = parse_input(TESTDATA)?;
        assert_eq!(143, part_one(&rules, &pages));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (rules, pages) = parse_input(DATA)?;
        assert_eq!(5374, part_one(&rules, &pages));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (rules, pages) = parse_input(TESTDATA)?;
        assert_eq!(123, part_two(&rules, pages));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (rules, pages) = parse_input(DATA)?;
        assert_eq!(4260, part_two(&rules, pages));

        Ok(())
    }
}
