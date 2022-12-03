use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day3, part1)]
pub fn part1(v: &str) -> i64 {
    v.lines()
        .into_iter()
        .map(|s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            let c = s1.chars().find(|c| s2.contains(*c)).unwrap();
            if c.is_lowercase() {
                c as i64 - 96
            } else {
                c as i64 - 38
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(v: &str) -> i64 {
    v.lines()
        .tuples()
        .into_iter()
        .map(|(l1, l2, l3)| l1.chars().find(|c| l2.contains(*c) && l3.contains(*c)))
        .map(Option::unwrap)
        .map(|c| {
            if c.is_lowercase() {
                c as i64 - 96
            } else {
                c as i64 - 38
            }
        })
        .sum()
}
