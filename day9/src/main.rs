use anyhow::Result;
use std::iter::repeat_n;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: Vec<Vec<Option<usize>>>) -> usize {
    let input = input.into_iter().flatten().collect::<Vec<Option<usize>>>();
    let mut vec = vec![];
    let mut length = input.len();
    let mut i = 0;
    while i < length {
        if let Some(id) = input[i] {
            vec.push(id);
        } else {
            loop {
                length -= 1;
                if let Some(id) = input[length] {
                    vec.push(id);
                    break;
                }
                if length < i {
                    break;
                }
            }
        }
        i += 1;
    }

    vec.iter().enumerate().map(|(i, id)| i * *id).sum()
}

fn part_two(mut input: Vec<Vec<Option<usize>>>) -> usize {
    for idx in 0..input.len() {
        let i = input.len() - idx - 1;
        if input[i][0].is_some() {
            for j in 0..i {
                let hole_length = input[j].len();
                let content_length = input[i].len();
                if input[j][0].is_none() && hole_length >= content_length {
                    input.swap(j, i);
                    if hole_length > content_length {
                        input[i].truncate(content_length);
                        input.insert(
                            j + 1,
                            repeat_n(None, hole_length - content_length).collect(),
                        );
                    }
                    break;
                }
            }
        }
    }

    input
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, id)| i * id.unwrap_or(0))
        .sum()
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<Option<usize>>>> {
    let mut vec = vec![];
    let mut id = 0;
    let mut empty = false;
    for c in input.chars() {
        let d = c.to_digit(10).unwrap() as usize;
        empty = !empty;
        if d == 0 {
            continue;
        }
        if empty {
            vec.push(repeat_n(Some(id), d).collect::<Vec<Option<usize>>>());
            id += 1;
        } else {
            vec.push(repeat_n(None, d).collect::<Vec<Option<usize>>>());
        }
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(1928, part_one(parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(6421128769094, part_one(parse_input(DATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(2858, part_two(parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(6448168620520, part_two(parse_input(DATA)?));

        Ok(())
    }
}
