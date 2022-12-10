use aoc_runner_derive::aoc;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Display, FromStr, Debug, Clone, Copy)]
pub enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Display, FromStr, Debug)]
#[display("{direction} {distance}")]
pub struct Instruction {
    direction: Direction,
    distance: i32,
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let ins = input
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect_vec();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();

    for i in ins {
        for _ in 0..i.distance {
            let dir = i.direction;
            match dir {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }

            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            if !x_range.cartesian_product(y_range).any(|i| i == tail) {
                tail = head;
                match dir {
                    Direction::Up => tail.1 -= 1,
                    Direction::Down => tail.1 += 1,
                    Direction::Left => tail.0 += 1,
                    Direction::Right => tail.0 -= 1,
                }
            };
        }

        visited.insert(tail);
    }

    visited.len()
}

// #[aoc(day9, part2)]
// pub fn part2(input: &str) -> usize {
//     let ins = input
//         .lines()
//         .map(Instruction::from_str)
//         .map(Result::unwrap)
//         .collect_vec();

//     let mut rope = [(0, 0); 10];
//     let mut visited = HashSet::from([*rope.last().unwrap()]);

//     for i in ins {
//         for _ in 0..i.distance {
//             let mut head = rope[0];
//             let mut tail = rope[rope.len() - 1];
//             {
//                 let dir = i.direction;
//                 match dir {
//                     Direction::Up => head.1 += 1,
//                     Direction::Down => head.1 -= 1,
//                     Direction::Left => head.0 -= 1,
//                     Direction::Right => head.0 += 1,
//                 }

//                 let x_range = (head.0 - 1)..=(head.0 + 1);
//                 let y_range = (head.1 - 1)..=(head.1 + 1);

//                 if !x_range.cartesian_product(y_range).any(|i| i == tail) {
//                     tail = head;
//                     match dir {
//                         Direction::Up => tail.1 -= 1,
//                         Direction::Down => tail.1 += 1,
//                         Direction::Left => tail.0 += 1,
//                         Direction::Right => tail.0 -= 1,
//                     }
//                 };
//             }
//         }

//         visited.insert(*rope.last().unwrap());
//     }

//     visited.len()
// }
