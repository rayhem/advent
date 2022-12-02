use itertools::Itertools;
use utils::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        let elf_calories = parse_calories(input);

        elf_calories.iter().max().map(i32::to_string)
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            parse_calories(input)
                .iter()
                .sorted()
                .rev()
                .take(3)
                .sum::<i32>()
                .to_string(),
        )
    }
}

fn parse_calories(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s| s.parse::<i32>())
        .group_by(|x| x.is_ok())
        .into_iter()
        .map(|(_, xs)| xs.flatten().sum())
        .collect()
}

#[cfg(test)]
mod test {
    mod unit {}

    mod integration {}
}
