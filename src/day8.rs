use aoc_runner_derive::aoc;

#[aoc(day8, part2)]
pub fn part1(input: &str) -> usize {
    let trees = input
        .lines()
        .map(|line| line.chars().map(|t| t.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut max_score = 1;
    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut left_score = 0;
            for left_tree in row[..j].iter().rev() {
                left_score += 1;
                if *left_tree >= *tree {
                    break;
                }
            }

            let mut right_score = 0;
            for right_tree in row[j + 1..].iter() {
                right_score += 1;
                if *right_tree >= *tree {
                    break;
                }
            }

            let mut below_score = 0;
            for below_row in trees[i + 1..].iter() {
                below_score += 1;
                if below_row[j] >= *tree {
                    break;
                }
            }

            let mut above_score = 0;

            for above_row in trees[..i].iter().rev() {
                above_score += 1;
                if above_row[j] >= *tree {
                    break;
                }
            }

            let score = left_score * right_score * below_score * above_score;

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}
