use itertools::Itertools;
use utils::puzzle::PuzzleImpl;

pub struct Day03 {}

impl PuzzleImpl for Day03 {
    type ParsedInput = String;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, utils::error::Error> {
        Ok(input.to_owned())
    }

    fn part_one(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        let total = input
            .lines()
            .map(|l| {
                let (a, b) = l.split_at(l.len() / 2);
                priority(shared_letter(a, b))
            })
            .sum::<i32>();

        Ok(total.to_string())
    }

    fn part_two(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        Err(utils::error::Error::Unimplemented)
    }
}

fn shared_letter(a: &str, b: &str) -> char {
    a.chars()
        .filter(|ch| b.contains(*ch))
        .next()
        .expect("No unique letters")
}

fn priority(c: char) -> i32 {
    (c as i32)
        - if c.is_lowercase() {
            ('a' as i32) - 1
        } else {
            ('A' as i32) - 27
        }
}

#[cfg(test)]
mod test {
    mod unit {}

    mod integration {}
}
