use anyhow::Result;
use std::collections::HashSet;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let (start, grid) = result?;

    let (took, result) = took::took(|| part_one(start, &grid));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let (start, mut grid) = result?;

    let (took, result) = took::took(|| part_two(start, &mut grid));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(start: State, grid: &[Vec<bool>]) -> usize {
    walked_positions(start, grid).len()
}

fn walked_positions(start: State, grid: &[Vec<bool>]) -> HashSet<(usize, usize)> {
    let mut state = start;
    let bounds = (grid[0].len(), grid.len());
    let mut set = HashSet::new();
    set.insert((state.x, state.y));
    while let Some(new_state) = step(&state, grid, bounds) {
        state = new_state;
        set.insert((state.x, state.y));
    }

    set
}

fn step(state: &State, grid: &[Vec<bool>], bounds: (usize, usize)) -> Option<State> {
    match state.make_move(bounds) {
        None => None,
        Some(new_state) => {
            if grid[new_state.y][new_state.x] {
                let right_state = state.turn_right();
                Some(right_state)
            } else {
                Some(new_state)
            }
        }
    }
}

fn part_two(start: State, grid: &mut [Vec<bool>]) -> usize {
    let walked_positions = walked_positions(start.clone(), grid);
    let mut count = 0;
    let mut last_change = (start.x, start.y);

    'outer: for block in walked_positions {
        grid[last_change.1][last_change.0] = false;
        grid[block.1][block.0] = true;
        last_change = block;

        let mut state = start.clone();
        let bounds = (grid[0].len(), grid.len());
        let mut set = vec![];
        while let Some(new_state) = step(&state, grid, bounds) {
            if new_state.direction != state.direction {
                if set.contains(&new_state) {
                    count += 1;
                    continue 'outer;
                }
                set.push(new_state.clone());
            }
            state = new_state;
        }
    }

    count
}

#[derive(Clone, Debug, PartialEq)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

impl State {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn turn_right(&self) -> Self {
        Self::new(self.x, self.y, self.direction.turn_right())
    }

    pub fn make_move(&self, bounds: (usize, usize)) -> Option<Self> {
        match self.direction {
            Direction::Up => {
                if self.y == 0 {
                    None
                } else {
                    Some(State::new(self.x, self.y - 1, self.direction))
                }
            }
            Direction::Right => {
                if self.x >= bounds.0 - 1 {
                    None
                } else {
                    Some(State::new(self.x + 1, self.y, self.direction))
                }
            }
            Direction::Down => {
                if self.y >= bounds.1 - 1 {
                    None
                } else {
                    Some(State::new(self.x, self.y + 1, self.direction))
                }
            }
            Direction::Left => {
                if self.x == 0 {
                    None
                } else {
                    Some(State::new(self.x - 1, self.y, self.direction))
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn parse_input(input: &'static str) -> Result<(State, Vec<Vec<bool>>)> {
    let mut start = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| {
                    if c == '^' {
                        start = Some(State::new(x, y, Direction::Up));
                    }
                    c == '#'
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    Ok((start.unwrap(), grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let (start, grid) = parse_input(TESTDATA)?;
        assert_eq!(41, part_one(start, &grid));

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (start, grid) = parse_input(DATA)?;
        assert_eq!(5129, part_one(start, &grid));

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (start, mut grid) = parse_input(TESTDATA)?;
        assert_eq!(6, part_two(start, &mut grid));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (start, mut grid) = parse_input(DATA)?;
        assert_eq!(1888, part_two(start, &mut grid));

        Ok(())
    }
}
