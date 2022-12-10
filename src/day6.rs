use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

pub fn calculate(input: &str, num: usize) -> usize {
    input
        .chars()
        .collect_vec()
        .windows(num)
        .position(|x| HashSet::<&char>::from_iter(x).len() == num)
        .unwrap()
        + num
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    calculate(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    calculate(input, 14)
}
