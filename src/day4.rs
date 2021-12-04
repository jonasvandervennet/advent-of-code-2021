use crate::util::{print_part_1, print_part_2};
use std::fs::read_to_string;
use std::time::Instant;

fn sum_unmarked(numbers: &[usize], board: &[[usize; 5]; 5]) -> usize {
    let mut sum = 0;
    board.iter().for_each(|line| {
        line.iter().for_each(|num| {
            if !numbers.contains(num) {
                sum += num
            }
        })
    });
    sum
}

fn board_wins(numbers: &[usize], board: &[[usize; 5]; 5]) -> bool {
    if board
        .iter()
        .any(|line| line.iter().all(|num| numbers.contains(num)))
    {
        return true;
    }

    for col in 0..board[0].len() {
        if board.iter().all(|line| numbers.contains(&line[col])) {
            return true;
        }
    }

    false
}

fn play_bingo(numbers: &[usize], boards: &[[[usize; 5]; 5]], find_first: bool) -> usize {
    let mut filtered_boards = vec![];

    for draw_index in 0..numbers.len() {
        let drawn_numers = &numbers[..draw_index + 1];
        for (board_index, board) in boards.iter().enumerate() {
            if filtered_boards.contains(&board_index) {
                continue;
            }
            if board_wins(drawn_numers, board) {
                if find_first || filtered_boards.len() == boards.len() - 1 {
                    return sum_unmarked(drawn_numers, board) * numbers[draw_index];
                } else {
                    filtered_boards.push(board_index)
                }
            }
        }
    }
    unreachable!();
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<[[usize; 5]; 5]>) {
    let mut input_lines = input.lines();

    let numbers: Vec<usize> = input_lines
        .next()
        .expect("Cannot parse input")
        .split(",")
        .map(|num| num.parse().expect("Cannot parse input"))
        .collect();

    let mut boards: Vec<[[usize; 5]; 5]> = vec![];
    let mut board_row_index = 0;

    loop {
        let line = input_lines.next();
        if !line.is_some() {
            break;
        }

        let line = line.unwrap();
        if line.is_empty() {
            boards.push([[0; 5]; 5]);
            board_row_index = 0;
            continue;
        }

        line.split_whitespace()
            .map(|num| num.parse().expect("Cannot parse input"))
            .enumerate()
            .for_each(|(i, num)| boards.last_mut().unwrap()[board_row_index][i] = num);

        board_row_index += 1;
    }

    (numbers, boards)
}

pub fn main() {
    let (numbers, boards) =
        parse_input(&read_to_string("inputs/day4.txt").expect("Input not found.."));

    // PART 1
    let start = Instant::now();
    let known_answer = "51034";
    let part_1: usize = play_bingo(&numbers, &boards, true);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "5434";
    let part_2: usize = play_bingo(&numbers, &boards, false);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let (numbers, boards) =
            parse_input(&read_to_string("inputs/day4_test.txt").expect("Input not found.."));
        assert_eq!(play_bingo(&numbers, &boards, true), 4512);
    }
    #[test]
    fn test_example_2() {
        let (numbers, boards) =
            parse_input(&read_to_string("inputs/day4_test.txt").expect("Input not found.."));
        assert_eq!(play_bingo(&numbers, &boards, false), 1924);
    }
}
