use aoc_runner_derive::aoc;
use btoi::btoi;

#[aoc(day4, part1)]
pub fn part1(v: &str) -> i64 {
    v.lines().into_iter().fold(0, |acc, line| {
        let (left, right) = line.split_once(',').unwrap();
        let range1 = left.split_once('-').unwrap();
        let range2 = right.split_once('-').unwrap();

        let r1 = (
            range1.0.parse::<u16>().unwrap(),
            range1.1.parse::<u16>().unwrap(),
        );
        let r2 = (
            range2.0.parse::<u16>().unwrap(),
            range2.1.parse::<u16>().unwrap(),
        );

        if (r1.0 >= r2.0 && r1.1 <= r2.1) || (r2.0 >= r1.0 && r2.1 <= r1.1) {
            acc + 1
        } else {
            acc
        }
    })
}

#[aoc(day4, part2)]
pub fn part2(v: &str) -> i64 {
    v.lines().into_iter().fold(0, |acc, line| {
        let (r1, r2) = line.split_once(',').unwrap();
        let r1 = r1.split_once('-').unwrap();
        let r2 = r2.split_once('-').unwrap();

        let r1 = (
            btoi::<u16>(r1.0.as_bytes()).unwrap(),
            btoi::<u16>(r1.1.as_bytes()).unwrap(),
        );
        let r2 = (
            btoi::<u16>(r2.0.as_bytes()).unwrap(),
            btoi::<u16>(r2.1.as_bytes()).unwrap(),
        );

        // check if r1 and r2 have any overlap at all
        if r1.0 <= r2.1 && r2.0 <= r1.1 {
            acc + 1
        } else {
            acc
        }
    })
}
