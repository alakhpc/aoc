use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<i64>().unwrap()).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|g| g.lines().map(|x| x.parse::<i64>().unwrap()).sum::<i64>())
        .sorted_unstable()
        .rev()
        .take(3)
        .sum()
}
