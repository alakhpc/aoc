use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut l = line.chars();
            (l.next().unwrap(), l.nth(1).unwrap())
        })
        .map(|(x, y)| {
            let mut c = match x {
                'A' => match y {
                    'X' => 3,
                    'Y' => 6,
                    _ => 0,
                },
                'B' => match y {
                    'Y' => 3,
                    'Z' => 6,
                    _ => 0,
                },
                'C' => match y {
                    'Z' => 3,
                    'X' => 6,
                    _ => 0,
                },
                _ => 0,
            };

            c += match y {
                'X' => 1,
                'Y' => 2,
                'Z' => 3,
                _ => 0,
            };

            c
        })
        .sum()
}

#[aoc(day2, part2, Bytes)]
pub fn part2(input: &[u8]) -> i64 {
    input
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        // (opp, ' ', me, '\n')
        .fold(0, |mut acc, f| {
            match (f.0, f.2) {
                (b'A', b'X') => acc += 3,
                (b'A', b'Y') => acc += 4,
                (b'A', b'Z') => acc += 8,
                (b'B', b'X') => acc += 1,
                (b'B', b'Y') => acc += 5,
                (b'B', b'Z') => acc += 9,
                (b'C', b'X') => acc += 2,
                (b'C', b'Y') => acc += 6,
                (b'C', b'Z') => acc += 7,
                _ => (),
            }
            acc
        })
}
