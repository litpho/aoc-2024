use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::pair,
    IResult, Parser,
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

fn part_one(input: &[Machine]) -> usize {
    input.iter().filter_map(|m| m.solve_one()).sum()
}

fn part_two(input: &[Machine]) -> usize {
    input.iter().filter_map(|m| m.solve_two()).sum()
}

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl Machine {
    pub fn solve_one(&self) -> Option<usize> {
        let mut solutions = vec![];
        for a in 0..=100 {
            let total_a_x = a * self.button_a.0;
            let total_a_y = a * self.button_a.1;
            if let Some(remainder_x) = self.prize.0.checked_sub(total_a_x) {
                if let Some(remainder_y) = self.prize.1.checked_sub(total_a_y) {
                    if remainder_x % self.button_b.0 == 0
                        && remainder_y % self.button_b.1 == 0
                        && remainder_x / self.button_b.0 == remainder_y / self.button_b.1
                    {
                        let b = remainder_x / self.button_b.0;
                        solutions.push((a, b));
                    }
                }
            }
        }

        solutions.iter().map(|(a, b)| a * 3 + b).min()
    }

    pub fn solve_two(&self) -> Option<usize> {
        let mut solutions = vec![];
        let prize = (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000);
        for a in 0.. {
            let total_a_x = a * self.button_a.0;
            let total_a_y = a * self.button_a.1;
            if let Some(remainder_x) = prize.0.checked_sub(total_a_x) {
                if let Some(remainder_y) = prize.1.checked_sub(total_a_y) {
                    if remainder_x % self.button_b.0 == 0
                        && remainder_y % self.button_b.1 == 0
                        && remainder_x / self.button_b.0 == remainder_y / self.button_b.1
                    {
                        let b = remainder_x / self.button_b.0;
                        println!("Solution for {self:?} -> {a},{b}");
                        solutions.push((a, b));
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        solutions.iter().map(|(a, b)| a * 3 + b).min()
    }
}

fn parse_input(input: &'static str) -> Result<Vec<Machine>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(pair(line_ending, line_ending), parse_machine).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        (
            parse_button,
            line_ending,
            parse_button,
            line_ending,
            parse_prize,
        ),
        |(button_a, _, button_b, _, prize)| Machine {
            button_a,
            button_b,
            prize,
        },
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        (
            tag("Button "),
            one_of("AB"),
            tag(": X+"),
            complete::u32,
            tag(", Y+"),
            complete::u32,
        ),
        |(_, _, _, x, _, y)| (x as usize, y as usize),
    )
    .parse(input)
}

fn parse_prize(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        (tag("Prize: X="), complete::u32, tag(", Y="), complete::u32),
        |(_, x, _, y)| (x as usize, y as usize),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 480);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 29877);

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(part_two(&parse_input(TESTDATA)?), 480);
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(parse_input(DATA)), 266820198587914);
    // }
}
