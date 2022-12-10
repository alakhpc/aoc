use aoc_runner_derive::aoc;
use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Display, FromStr, Debug)]
pub enum Instruction {
    #[display("$ cd {dir}")]
    Cd { dir: String },
    #[display("$ ls")]
    Ls,
    #[display("dir {name}")]
    Dir { name: String },
    #[display("{size} {name}")]
    File { name: String, size: usize },
}

fn full_path(cdir: &[String], name: &str) -> String {
    let mut path = cdir.join("/");

    if !path.is_empty() {
        path.push('/');
    }

    path.push_str(name);
    path
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let mut files = HashMap::<String, usize>::new();
    let mut dirs = HashSet::<String>::new();
    let mut cdir = Vec::<String>::new();

    input
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .for_each(|i| match i {
            Instruction::Cd { dir } => {
                if dir == ".." {
                    cdir.pop();
                } else {
                    cdir.push(dir);
                }
            }
            Instruction::File { name, size } => {
                files.insert(full_path(&cdir, &name), size);
            }
            Instruction::Dir { name } => {
                dirs.insert(full_path(&cdir, &name));
            }
            Instruction::Ls => {}
        });

    dirs.iter()
        .map(|d| {
            files
                .iter()
                .filter(|(n, _)| n.starts_with(d))
                .map(|(_, s)| s)
                .sum::<usize>()
        })
        .filter(|s| *s <= 100000)
        .sum::<usize>()
}
