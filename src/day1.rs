use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn count_rising_in_window_of_2(values: &[usize]) -> usize {
    let mut sum = 0;
    for window in values.windows(2) {
        if window[0] < window[1] {
            sum += 1;
        }
    }
    sum
}

fn count_rising_sum_in_window_of_3(values: &[usize]) -> usize {
    // count rising over sums per 3
    count_rising_in_window_of_2(
        &values
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<usize>>(),
    )
}

pub fn main() {
    let input: Vec<usize> = read_to_string("inputs/day1.txt")
        .expect("Input not found..")
        .lines()
        .map(|line| line.parse::<usize>().expect("Could not decode input.."))
        .collect();

    // PART 1
    let start = Instant::now();
    let known_answer = "1692";
    let part_1: usize = count_rising_in_window_of_2(&input);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "1724";
    let part_2: usize = count_rising_sum_in_window_of_3(&input);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_rising_in_window_of_2(&input), 7);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_rising_sum_in_window_of_3(&input), 5);
    }
}
