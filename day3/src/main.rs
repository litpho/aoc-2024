const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse_input(DATA, false));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| solve(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA, true));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| solve(&input));
    println!("Result part two: {result}");
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
    fn test_part_two_testdata() {
        assert_eq!(48, solve(&parse_input(TESTDATA2, true)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(82868252, solve(&parse_input(DATA, true)));
    }
}
