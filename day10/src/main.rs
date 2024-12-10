use anyhow::Result;
use itertools::Itertools;

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

fn part_one(grid: &[Vec<usize>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0)
                .map(move |(x, _)| count_trailhead_score(grid, x, y))
        })
        .sum()
}

fn count_trailhead_score(grid: &[Vec<usize>], x: usize, y: usize) -> usize {
    println!("Trailheads: {x},{y}");
    let count = count_trailheads_internal(grid, x, y, vec![], vec![], 0)
        .into_iter()
        .flatten()
        .unique()
        .count();

    println!("Trailheads: ({x},{y}) - {count}");
    count
}

fn count_trailheads_internal(
    grid: &[Vec<usize>],
    x: usize,
    y: usize,
    visited: Vec<(usize, usize)>,
    heights: Vec<usize>,
    height: usize,
) -> Vec<Vec<(usize, usize)>> {
    if grid[y][x] != height {
        return vec![];
    }
    if height == 9 {
        // println!("Checking {x},{y} for {visited:?} at height {height} - {heights:?}");
        return vec![vec![(x, y)]];
    }

    let bounds = (grid[0].len() - 1, grid.len() - 1);
    let mut result = vec![];

    if y > 0 && !visited.contains(&(x, y - 1)) {
        // up
        let mut vec = visited.clone();
        vec.push((x, y));
        let mut vec2 = heights.clone();
        vec2.push(height + 1);
        count_trailheads_internal(grid, x, y - 1, vec, vec2, height + 1)
            .into_iter()
            .flatten()
            .for_each(|x| result.push(x));
    }
    if x < bounds.0 && !visited.contains(&(x + 1, y)) {
        // right
        let mut vec = visited.clone();
        vec.push((x, y));
        let mut vec2 = heights.clone();
        vec2.push(height + 1);
        count_trailheads_internal(grid, x + 1, y, vec, vec2, height + 1)
            .into_iter()
            .flatten()
            .for_each(|x| result.push(x));
    }
    if y < bounds.1 && !visited.contains(&(x, y + 1)) {
        // down
        let mut vec = visited.clone();
        vec.push((x, y));
        let mut vec2 = heights.clone();
        vec2.push(height + 1);
        count_trailheads_internal(grid, x, y + 1, vec, vec2, height + 1)
            .into_iter()
            .flatten()
            .for_each(|x| result.push(x));
    }
    if x > 0 && !visited.contains(&(x - 1, y)) {
        // left
        let mut vec = visited.clone();
        vec.push((x, y));
        let mut vec2 = heights.clone();
        vec2.push(height + 1);
        count_trailheads_internal(grid, x - 1, y, vec, vec2, height + 1)
            .into_iter()
            .flatten()
            .for_each(|x| result.push(x));
    }

    vec![result]
}

fn part_two(_input: &[Vec<usize>]) -> usize {
    unimplemented!()
}

fn parse_input(input: &'static str) -> Result<Vec<Vec<usize>>> {
    let vec = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(36, part_one(&parse_input(TESTDATA)?));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(461, part_one(&parse_input(DATA)?));

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     assert_eq!(2858, part_two(parse_input(TESTDATA)?));
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_part_two() -> Result<()> {
    //     assert_eq!(6448168620520, part_two(parse_input(DATA)?));
    //
    //     Ok(())
    // }
}
