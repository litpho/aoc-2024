use std::collections::HashMap;
use itertools::{repeat_n, Itertools};

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    // let (took, result) = took::took(|| parse_input(DATA));
    // println!("Time spent parsing: {took}");
    // let input = result;
    //
    // let (took, result) = took::took(|| part_two(&input));
    // println!("Result part two: {result}");
    // println!("Time spent: {took}");
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let corners = calculate_corners(grid);

    // for corner in corners {
    //     println!("Corner: {corner:?}");
    // }

    calc_perimeter(corners.get(&'C').unwrap());

    unimplemented!()
}

fn calculate_corners(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Corner>> {
    let mut corners: HashMap<char, Vec<Corner>> = HashMap::new();
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let (curr, top, right, bottom, left) = (grid[y][x], grid[y - 1][x], grid[y][x + 1], grid[y + 1][x], grid[y][x - 1]);

            if curr != top && curr != left {
                corners.entry(curr).or_default().push(Corner::new(x, y, Dir::TopLeft));
            }
            if curr != top && curr != right {
                corners.entry(curr).or_default().push(Corner::new(x, y, Dir::TopRight));
            }
            if curr != bottom && curr != left {
                corners.entry(curr).or_default().push(Corner::new(x, y, Dir::BottomLeft));
            }
            if curr != bottom && curr != right {
                corners.entry(curr).or_default().push(Corner::new(x, y, Dir::BottomRight));
            }
        }
    }

    corners
}

fn calc_perimeter(corners: &[Corner]) -> (usize, Vec<Corner>) {
    let mut count = 0;
    let mut remainder = vec![];
    let first = &corners[0];
    let mut next = first;
    loop {
        println!("Moving from {next:?}");
        next = next_corner(corners, next);
        println!("to {next:?}");
        if next.eq(first) {
            break;
        }
    }

    (count, remainder)
}

fn next_corner<'a>(corners: &'a[Corner], corner: &'a Corner) -> &'a Corner {
    match corner.dir {
        Dir::TopLeft => corners.iter()
            .sorted_by(|c1, c2| c1.x.cmp(&c2.x))
            .find(|c| c.y == corner.y && c.x >= corner.x && [Dir::TopRight, Dir::BottomRight].contains(&c.dir))
            .unwrap(),
        Dir::TopRight => corners.iter()
            .sorted_by(|c1, c2| c1.y.cmp(&c2.y))
            .find(|c| c.x == corner.x && c.y >= corner.y && [Dir::BottomRight, Dir::BottomLeft].contains(&c.dir))
            .unwrap(),
        Dir::BottomRight => corners.iter()
            .sorted_by(|c1, c2| c2.y.cmp(&c1.y))
            .find(|c| c.y == corner.y && c.x <= corner.x && [Dir::BottomLeft, Dir::TopLeft].contains(&c.dir))
            .unwrap(),
        Dir::BottomLeft => corners.iter()
            .sorted_by(|c1, c2| c2.x.cmp(&c1.x))
            .find(|c| c.x == corner.x && c.y <= corner.y && [Dir::TopRight, Dir::BottomRight].contains(&c.dir))
            .unwrap(),
    }
}

// fn part_two(input: &[Vec<char>]) -> usize {
//     unimplemented!()
// }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq, Eq)]
struct Corner {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Corner {
    pub fn new(x: usize, y: usize, dir: Dir) -> Corner {
        Corner { x, y, dir }
    }
}

fn parse_input(input: &'static str) -> Vec<Vec<char>> {
    let mut vec = input
        .lines()
        .map(|line| {
            let mut vec = line.chars().collect::<Vec<char>>();
            vec.insert(0, '.');
            vec.push('.');

            vec
        })
        .collect::<Vec<Vec<char>>>();
    let line = repeat_n('.', vec[0].len()).collect::<Vec<char>>();
    vec.insert(0, line.clone());
    vec.push(line);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_small() {
        let data = "AAAA\nBBCD\nBBCC\nEEEC";
        assert_eq!(part_one(&parse_input(data)), 140);
    }

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(part_one(&parse_input(TESTDATA)), 1930);
    }

    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one(parse_input(DATA)), 224529);
    // }
    //
    // #[test]
    // fn test_part_two_testdata() {
    //     assert_eq!(part_two(parse_input(DATA)), 266820198587914);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(parse_input(DATA)), 266820198587914);
    // }
}
