use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn part1(values: &[Vec<bool>]) -> usize {
    let num_entries = values.len();
    let num_bits = values[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_index in 0..num_bits {
        let value = usize::pow(2, (num_bits - 1 - bit_index) as u32);

        let count: usize = values
            .iter()
            .map(|entry| if entry[bit_index] { 1 } else { 0 })
            .sum();

        if count >= num_entries / 2 {
            gamma += value;
        } else {
            epsilon += value;
        }
    }

    gamma * epsilon
}

fn bin2dec(bits: &[bool]) -> usize {
    let num_bits = bits.len();
    bits.iter()
        .enumerate()
        .map(|(bit_index, &bit)| {
            if bit {
                usize::pow(2, (num_bits - 1 - bit_index) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn calc_p2(_values: &Vec<Vec<bool>>, choose_most_occuring: bool) -> usize {
    let mut values = _values.to_vec();

    let mut bit_index = 0;

    while values.len() > 1 {
        let count: usize = values
            .iter()
            .map(|entry| if entry[bit_index] { 1 } else { 0 })
            .sum();

        let num_entries = values.len();

        let most_occuring = count * 2 >= num_entries;

        values = values
            .iter()
            .filter(|bits| {
                if choose_most_occuring {
                    bits[bit_index] == most_occuring
                } else {
                    bits[bit_index] != most_occuring
                }
            })
            .map(|bits| bits.to_vec())
            .collect::<Vec<Vec<bool>>>()
            .to_vec();
        bit_index += 1;
    }

    bin2dec(&values[0])
}

fn part2(values: &Vec<Vec<bool>>) -> usize {
    calc_p2(values, true) * calc_p2(values, false)
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

pub fn main() {
    let input = parse_input(&read_to_string("inputs/day3.txt").expect("Input not found.."));

    // PART 1
    let start = Instant::now();
    let known_answer = "3320834";
    let part_1: usize = part1(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "4481199";
    let part_2: usize = part2(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(part1(&parse_input(input)), 198);
    }

    #[test]
    fn test_example_2() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(part2(&parse_input(input)), 230);
    }
}
