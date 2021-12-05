use crate::util::{print_part_1, print_part_2};
use std::cmp;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn count_overlap(points: &[(Point, Point)], diagonals: bool) -> usize {
    let max_x = points
        .iter()
        .map(|(begin, end)| cmp::max(begin.x, end.x))
        .max()
        .expect("Invalid maxX");

    let max_y = points
        .iter()
        .map(|(begin, end)| cmp::max(begin.y, end.y))
        .max()
        .expect("Invalid maxY");

    let mut grid: Vec<Vec<usize>> = vec![vec![0; max_x + 1]; max_y + 1];

    points
        .iter()
        .filter(|(begin, end)| diagonals || begin.x == end.x || begin.y == end.y)
        .for_each(|(begin, end)| {
            let mut x = begin.x;
            let mut y = begin.y;

            loop {
                grid[y][x] += 1;

                if x == end.x && y == end.y {
                    break;
                }

                if x < end.x {
                    x += 1;
                } else if x > end.x {
                    x -= 1;
                }
                if y < end.y {
                    y += 1;
                } else if y > end.y {
                    y -= 1;
                }
            }
        });

    grid.iter()
        .map(|line| line.iter().filter(|&&count| count > 1).count())
        .sum()
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .map(|line| {
            let points: Vec<Point> = line
                .split(" -> ")
                .map(|coords| {
                    let xy: Vec<usize> = coords
                        .split(",")
                        .map(|num| num.parse().expect("Cannot parse input"))
                        .collect();
                    Point { x: xy[0], y: xy[1] }
                })
                .collect();
            (points[0], points[1])
        })
        .collect()
}

pub fn main() {
    let input = parse_input(&read_to_string("inputs/day5.txt").expect("Input not found.."));

    // PART 1
    let start = Instant::now();
    let known_answer = "5147";
    let part_1: usize = count_overlap(&input, false);
    let duration = start.elapsed();
    print_part_1(&part_1.to_string(), &known_answer, duration);

    // PART 2
    let start = Instant::now();
    let known_answer = "16925";
    let part_2: usize = count_overlap(&input, true);
    let duration = start.elapsed();
    print_part_2(&part_2.to_string(), &known_answer, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input =
            parse_input(&read_to_string("inputs/day5_test.txt").expect("Input not found.."));
        assert_eq!(count_overlap(&input, false), 5);
    }

    #[test]
    fn test_example_2() {
        let input =
            parse_input(&read_to_string("inputs/day5_test.txt").expect("Input not found.."));
        assert_eq!(count_overlap(&input, true), 12);
    }
}
