use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {took}");
    let (map, instructions) = result?;

    let (took, result) = took::took(|| part_one(map, &instructions));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    // let (took, result) = took::took(|| parse_input(DATA));
    // println!("Time spent parsing: {took}");
    // let input = result;
    //
    // let (took, result) = took::took(|| part_two(input));
    // println!("Result part two: {result}");
    // println!("Time spent: {took}");

    Ok(())
}

fn part_one(mut map: Map, instructions: &[Instruction]) -> usize {
    println!("Position {:?}", map.position);
    for instruction in instructions {
        map.handle(instruction);
        println!("{:?}", map.position);
    }

    map.count()
}

// fn part_two(input: HashMap<usize, usize>) -> usize {
//     solve(input, 75)
// }

#[derive(Debug)]
struct Map {
    position: (usize, usize),
    content: Vec<Vec<Item>>,
}

impl Map {
    pub fn new(mut content: Vec<Vec<Item>>) -> Self {
        let mut position = (0, 0);
        for (y, row) in content.iter_mut().enumerate() {
            if let Some((x, _)) = row.iter().find_position(|item| Item::Robot.eq(*item)) {
                position = (x, y);
                row[x] = Item::Floor;
                break;
            }
        }

        Map { content, position }
    }

    pub fn handle(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Up => {
                if self.content[self.position.1 - 1][self.position.0] == Item::Wall {
                    return;
                }

                let (pos, item) = (0..(self.position.1 - 1)).rev()
                    .position(|y| {
                        [Item::Wall, Item::Floor].contains(&self.content[y][self.position.0])
                    })
                    .map(|y| (y, &self.content[y][self.position.0]))
                    .unwrap();

                if Item::Wall.eq(item) {
                    return;
                }

                self.position.1 -= 1;
                (self.position.1..pos).rev().for_each(|y| self.content[y-1][self.position.0] = Item::Box);
            }
            Instruction::Down => {
                if self.content[self.position.1 + 1][self.position.0] == Item::Wall {
                    return;
                }

                self.position.1 += 1;
            }
            Instruction::Left => {
                if self.content[self.position.1][self.position.0 - 1] == Item::Wall {
                    return;
                }

                self.position.0 -= 1;
            }
            Instruction::Right => {
                if self.content[self.position.1][self.position.0 + 1] == Item::Wall {
                    return;
                }

                self.position.1 += 1;
            }
        }
    }

    pub fn count(self) -> usize {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, item)| {
                    if Item::Floor.eq(item) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .map(|(x, y)| x * 100 + y)
            .sum()
    }
}

#[derive(Debug, PartialEq)]
enum Item {
    Robot,
    Box,
    Wall,
    Floor,
}

impl TryFrom<char> for Item {
    type Error = ();

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        Ok(match c {
            '@' => Item::Robot,
            'O' => Item::Box,
            '#' => Item::Wall,
            '.' => Item::Floor,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        Ok(match c {
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            _ => return Err(()),
        })
    }
}

fn parse_input(input: &'static str) -> Result<(Map, Vec<Instruction>)> {
    let (_, input) = parse(input)?;

    Ok(input)
}

fn parse(input: &str) -> IResult<&str, (Map, Vec<Instruction>)> {
    separated_pair(
        parse_map,
        pair(line_ending, line_ending),
        parse_instructions,
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    map(separated_list1(line_ending, parse_map_line), |content| {
        Map::new(content)
    })(input)
}

fn parse_map_line(input: &str) -> IResult<&str, Vec<Item>> {
    many1(map_res(one_of("@O#."), |c| Item::try_from(c)))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    map(
        separated_list1(line_ending, many1(parse_instruction)),
        |v| v.into_iter().flatten().collect(),
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(one_of("^v<>"), |c| Instruction::try_from(c))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TESTDATA: &str = include_str!("test.txt");
    const TESTDATA_SMALL: &str = include_str!("test2.txt");

    #[test]
    fn test_part_one_testdata_small() -> Result<()> {
        let (map, instructions) = parse_input(TESTDATA_SMALL)?;
        assert_eq!(part_one(map, &instructions), 2028);

        Ok(())
    }

    // #[test]
    // fn test_part_one_testdata() {
    //     assert_eq!(part_one(parse_input(TESTDATA)), 10092);
    // }

    // #[test]
    // fn test_part_one() {
    //     assert_eq!(part_one(parse_input(DATA)), 224529);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(parse_input(DATA)), 266820198587914);
    // }
}
