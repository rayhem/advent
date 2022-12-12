use itertools::Itertools;
use utils::{error::Error, puzzle::PuzzleImpl};

pub struct Day01 {}

impl PuzzleImpl for Day01 {
    type ParsedInput = Vec<i32>;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, Error> {
        Ok(input
            .lines()
            .map(|s| s.parse::<i32>())
            .group_by(|x| x.is_ok())
            .into_iter()
            .map(|(_, xs)| xs.flatten().sum::<i32>())
            .collect())
    }

    fn part_a(parsed_input: &Self::ParsedInput) -> Result<String, Error> {
        parsed_input
            .iter()
            .max()
            .map(i32::to_string)
            .ok_or(Error::ParseError)
    }

    fn part_b(parsed_input: &Self::ParsedInput) -> Result<String, Error> {
        Ok(parsed_input
            .iter()
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    mod unit {}
}

#[cfg(feature = "verify")]
utils::verify_submitted_answers!(Day01, 2022, 01, "67658", "200158");
