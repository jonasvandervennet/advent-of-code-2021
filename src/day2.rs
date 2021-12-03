use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

enum Direction {
    UP,
    DOWN,
    FORWARD,
}

fn part1(values: &[(Direction, usize)]) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    for (dir, amount) in values.iter() {
        match dir {
            Direction::UP => depth -= amount,
            Direction::DOWN => depth += amount,
            Direction::FORWARD => distance += amount,
        }
    }
    depth * distance
}

fn part2(values: &[(Direction, usize)]) -> usize {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for (dir, amount) in values.iter() {
        match dir {
            Direction::UP => aim -= amount,
            Direction::DOWN => aim += amount,
            Direction::FORWARD => {
                distance += amount;
                depth += aim * amount
            }
        }
    }
    depth * distance
}

fn parse_input(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = match parts[0] {
                "up" => Direction::UP,
                "down" => Direction::DOWN,
                "forward" => Direction::FORWARD,
                _ => unreachable!(),
            };
            let amount = parts[1].parse::<usize>().expect("Could not decode input..");
            (direction, amount)
        })
        .collect()
}

pub fn main() {
    let input = parse_input(&read_to_string("inputs/day2.txt").expect("Input not found.."));

    // PART 1
    let start = Instant::now();
    let known_answer = "2147104";
    let part_1: usize = part1(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "2044620088";
    let part_2: usize = part2(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(part1(&parse_input(input)), 150);
    }

    #[test]
    fn test_example_2() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(part2(&parse_input(input)), 900);
    }
}
