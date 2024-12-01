use itertools::Itertools;

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let mut input = result;

    let (took, result) = took::took(|| part_one(&mut input.0, &mut input.1));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input.0, input.1));
    println!("Result part two: {result}");
    println!("Time spent: {took}");
}

fn part_one(left: &mut [usize], right: &mut [usize]) -> usize {
    left.sort();
    right.sort();

    (0..left.len()).map(|i| left[i].abs_diff(right[i])).sum()
}

fn part_two(left: &[usize], right: Vec<usize>) -> usize {
    let map = right.iter().counts_by(|u| u);

    left.iter().map(|l| l * map.get(l).unwrap_or(&0)).sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        if let Some((lnum, rnum)) = line.split_once("   ") {
            left.push(lnum.parse::<usize>().unwrap());
            right.push(rnum.parse::<usize>().unwrap());
        }
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() {
        let mut input = parse(TESTDATA);
        assert_eq!(11, part_one(&mut input.0, &mut input.1));
    }

    #[test]
    fn test_part_one() {
        let mut input = parse(DATA);
        assert_eq!(2742123, part_one(&mut input.0, &mut input.1));
    }

    #[test]
    fn test_part_two_testdata() {
        let input = parse(TESTDATA);
        assert_eq!(31, part_two(&input.0, input.1));
    }

    #[test]
    fn test_part_two() {
        let input = parse(DATA);
        assert_eq!(21328497, part_two(&input.0, input.1));
    }
}