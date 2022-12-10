use aoc_runner_derive::aoc;
use itertools::Itertools;

fn format_top(array: Vec<Vec<char>>) -> String {
    let mut v = Vec::new();
    for mut arr in array {
        let c = arr.pop().unwrap();
        v.push(c);
    }

    v.into_iter().collect()
}

#[aoc(day5, part1)]
pub fn part1(v: &str) -> String {
    let mut array: [Vec<char>; 9] = Default::default();

    v.lines().into_iter().take(8).for_each(|line| {
        let chars = line.chars().collect_vec();
        match chars[1] {
            ' ' => {}
            x => array[0].push(x),
        };

        match chars[5] {
            ' ' => {}
            x => array[1].push(x),
        };

        match chars[9] {
            ' ' => {}
            x => array[2].push(x),
        };

        match chars[13] {
            ' ' => {}
            x => array[3].push(x),
        };

        match chars[17] {
            ' ' => {}
            x => array[4].push(x),
        };

        match chars[21] {
            ' ' => {}
            x => array[5].push(x),
        };

        match chars[25] {
            ' ' => {}
            x => array[6].push(x),
        };

        match chars[29] {
            ' ' => {}
            x => array[7].push(x),
        };

        match chars[33] {
            ' ' => {}
            x => array[8].push(x),
        };
    });

    array.iter_mut().for_each(|v| v.reverse());

    v.lines().into_iter().skip(10).for_each(|ins| {
        let reg = regex::Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
        let caps = reg.captures(ins).unwrap();
        let q = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let f = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let t = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let mut array_to_append = Vec::new();
        for _ in 0..q {
            let c = array[f - 1].pop().unwrap();
            array_to_append.push(c);
        }

        array_to_append.reverse();
        array[t - 1].append(&mut array_to_append);
    });

    format_top(array.to_vec())
}

// #[aoc(day5, part2)]
// pub fn part2(v: &str) -> i64 {
//     todo!()
// }

#[aoc(day5, part1, New)]
pub fn part12(v: &str) -> String {
    let num_stacks = v.lines().next().unwrap().len() / 4 + 1;
    let mut stacks = Vec::<Vec<char>>::new();
    stacks.resize_with(num_stacks, Vec::new);
    let mut iter = v.lines();

    iter.try_for_each(|line| -> Result<(), ()> {
        for (idx, stack) in stacks.iter_mut().enumerate() {
            let char = line.chars().nth((4 * idx) + 1).ok_or(())?;

            if char.is_ascii_digit() {
                return Err(());
            }

            if char.is_whitespace() {
                continue;
            }

            stack.push(char);
        }

        Ok(())
    })
    .ok();

    iter.skip(1).for_each(|ins| {
        let reg = regex::Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
        let caps = reg.captures(ins).unwrap();
        let q = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let f = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let t = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let mut array_to_append = Vec::new();
        for _ in 0..q {
            let c = stacks[f - 1].pop().unwrap();
            array_to_append.push(c);
        }

        array_to_append.reverse();
        stacks[t - 1].append(&mut array_to_append);
    });

    format_top(stacks)
}
