use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

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

fn part_one(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(goal, factors)| solveable_one(goal, factors))
        .map(|(goal, _)| goal)
        .sum()
}

fn solveable_one(goal: &u64, factors: &[u64]) -> bool {
    solve_internally_one(goal, factors, 0, Operator::Add)
        || solve_internally_one(goal, factors, 1, Operator::Multiply)
}

fn solve_internally_one(goal: &u64, factors: &[u64], subtotal: u64, operator: Operator) -> bool {
    let new_subtotal = match operator {
        Operator::Add => subtotal + factors[0],
        Operator::Multiply => subtotal * factors[0],
        _ => panic!("Unexpected operator"),
    };

    if subtotal > *goal {
        return false;
    }

    if factors.len() == 1 {
        return new_subtotal == *goal;
    }

    solve_internally_one(goal, &factors[1..], new_subtotal, Operator::Add)
        || solve_internally_one(goal, &factors[1..], new_subtotal, Operator::Multiply)
}

fn part_two(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(goal, factors)| solveable_two(goal, factors))
        .map(|(goal, _)| goal)
        .sum()
}

fn solveable_two(goal: &u64, factors: &[u64]) -> bool {
    solve_internally_two(goal, factors, 0, Operator::Add)
        || solve_internally_two(goal, factors, 1, Operator::Multiply)
        || solve_internally_two(goal, factors, 1, Operator::Concatenate)
}

fn solve_internally_two(goal: &u64, factors: &[u64], subtotal: u64, operator: Operator) -> bool {
    let new_subtotal = match operator {
        Operator::Add => subtotal + factors[0],
        Operator::Multiply => subtotal * factors[0],
        Operator::Concatenate => format!("{subtotal}{}", factors[0]).parse::<u64>().unwrap(),
    };

    if subtotal > *goal {
        return false;
    }

    if factors.len() == 1 {
        return new_subtotal == *goal;
    }

    solve_internally_two(goal, &factors[1..], new_subtotal, Operator::Add)
        || solve_internally_two(goal, &factors[1..], new_subtotal, Operator::Multiply)
        || solve_internally_two(goal, &factors[1..], new_subtotal, Operator::Concatenate)
}

enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn parse_input(input: &'static str) -> Result<Vec<(u64, Vec<u64>)>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(tag(" "), complete::u64),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    // #[test]
    // fn test_solveable() -> Result<()> {
    //     assert!(solveable(&3267, &[81, 40, 27]));
    //
    //     Ok(())
    // }

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(3749, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(2654749936343, part_one(&parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(11387, part_two(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(124060392153684, part_two(&parse_input(DATA)?));

        Ok(())
    }
}
