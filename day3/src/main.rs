use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::map,
    multi::{many0, many1, many_till},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse_input(DATA, false));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| solve(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_nom(DATA));
    println!("Time spent parsing: {took}");
    let (_, input) = result.unwrap();

    let (took, result) = took::took(|| part_one_nom(&input));
    println!("Result part one nom: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA, true));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| solve(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_nom(DATA));
    println!("Time spent parsing: {took}");
    let (_, input) = result.unwrap();

    let (took, result) = took::took(|| part_two_nom(&input));
    println!("Result part two nom: {result}");
    println!("Time spent: {took}");
}

fn solve(input: &[Mul]) -> u32 {
    input.iter().map(|m| m.a * m.b).sum()
}

#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}

fn parse_input(input: &'static str, has_ignore: bool) -> Vec<Mul> {
    let mut vec = vec![];
    let mut ignore = false;
    for i in 0..input.len() {
        if has_ignore && input[i..].starts_with("don't()") {
            ignore = true;
            continue;
        }

        if has_ignore && input[i..].starts_with("do()") {
            ignore = false;
            continue;
        }

        if ignore {
            continue;
        }

        if input[i..].starts_with("mul(") {
            if let Some(paren) = input[i..].find(')') {
                if let Some((a, b)) = input[i + 4..paren + i].split_once(',') {
                    let a = match a.parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => continue,
                    };

                    let b = match b.parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => continue,
                    };

                    vec.push(Mul { a, b });
                }
            }
        }
    }

    vec
}

#[derive(Debug)]
enum Instr {
    Mul(Mul),
    Do,
    Dont,
}

fn part_one_nom(input: &[Instr]) -> u32 {
    input
        .iter()
        .filter_map(|m| match m {
            Instr::Mul(m) => Some(m.a * m.b),
            Instr::Do => None,
            Instr::Dont => None,
        })
        .sum()
}

fn part_two_nom(input: &[Instr]) -> u32 {
    let mut result = 0;
    let mut ignore = false;
    for instr in input {
        match instr {
            Instr::Mul(mul) => {
                if !ignore {
                    result += mul.a * mul.b
                }
            }
            Instr::Do => ignore = false,
            Instr::Dont => ignore = true,
        }
    }

    result
}

fn parse_nom(input: &str) -> IResult<&str, Vec<Instr>> {
    terminated(many1(parse_instr), many0(anychar)).parse(input)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    map(
        many_till(anychar, alt((parse_do, parse_dont, parse_mul))),
        |(_, instr)| instr,
    )
    .parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Instr> {
    map(tag("do()"), |_| Instr::Do).parse(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instr> {
    map(tag("don't()"), |_| Instr::Dont).parse(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instr> {
    map(
        delimited(
            tag("mul("),
            separated_pair(complete::u32, tag(","), complete::u32),
            tag(")"),
        ),
        |(a, b)| Instr::Mul(Mul { a, b }),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA2: &str = include_str!("test2.txt");

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(161, solve(&parse_input(TESTDATA, false)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(170778545, solve(&parse_input(DATA, false)));
    }

    #[test]
    fn test_part_one_testdata_nom() {
        assert_eq!(161, part_one_nom(&parse_nom(TESTDATA).unwrap().1));
    }

    #[test]
    fn test_part_one_nom() {
        assert_eq!(170778545, part_one_nom(&parse_nom(DATA).unwrap().1));
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(48, solve(&parse_input(TESTDATA2, true)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(82868252, solve(&parse_input(DATA, true)));
    }

    #[test]
    fn test_part_two_testdata_nom() {
        assert_eq!(48, part_two_nom(&parse_nom(TESTDATA2).unwrap().1));
    }

    #[test]
    fn test_part_two_nom() {
        assert_eq!(82868252, part_two_nom(&parse_nom(DATA).unwrap().1));
    }
}
