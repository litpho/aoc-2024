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

fn part_one(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|report| is_safe(report)).count()
}

fn is_safe(report: &[usize]) -> bool {
    if report[0] == report[1] {
        return false;
    }

    let inc = report[0] < report[1];
    report.windows(2).all(|x| {
        let (a, b) = (x[0], x[1]);
        if inc {
            b > a && b - a <= 3
        } else {
            a > b && a - b <= 3
        }
    })
}

fn part_two(input: &[Vec<usize>]) -> usize {
    input.iter().filter(|report| is_safe_damper(report)).count()
}

fn is_safe_damper(report: &[usize]) -> bool {
    if is_safe(report) {
        return true;
    }

    (0..report.len())
        .map(|i| [&report[..i], &report[i + 1..]].concat())
        .any(|a| is_safe(&a))
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(2, part_one(&parse(TESTDATA)));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(252, part_one(&parse(DATA)));
    }

    #[test]
    fn test_part_two_testdata() {
        assert_eq!(4, part_two(&parse(TESTDATA)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(324, part_two(&parse(DATA)));
    }
}
