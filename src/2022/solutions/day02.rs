use utils::puzzle::PuzzleImpl;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Throw {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl TryFrom<char> for Throw {
    type Error = utils::error::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(utils::error::Error::ParseError),
        }
    }
}

impl Throw {
    fn loses_against(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draws_against(self) -> Self {
        self
    }

    fn wins_against(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameResult {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for GameResult {
    type Error = utils::error::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(utils::error::Error::ParseError),
        }
    }
}

fn game_score(p1: Throw, p2: Throw) -> i32 {
    (p2 as i32 + 1)
        + if p1 == p2 {
            3
        } else if p1.loses_against() == p2 {
            6
        } else {
            0
        }
}

fn game_score2(p1: Throw, result: GameResult) -> i32 {
    match result {
        GameResult::Lose => game_score(p1, p1.wins_against()),
        GameResult::Draw => game_score(p1, p1.draws_against()),
        GameResult::Win => game_score(p1, p1.loses_against()),
    }
}

fn tally_by(parsed_input: &Vec<[char; 2]>, tally: fn([char; 2]) -> i32) -> i32 {
    parsed_input.iter().map(|chars| tally(*chars)).sum::<i32>()
}

pub struct Day02 {}

impl PuzzleImpl for Day02 {
    type ParsedInput = Vec<[char; 2]>;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, utils::error::Error> {
        Ok(input
            .lines()
            .map(|c| {
                let mut chars = c.chars();
                [
                    chars.next().ok_or(utils::error::Error::ParseError).unwrap(),
                    chars.nth(1).ok_or(utils::error::Error::ParseError).unwrap(),
                ]
            })
            .collect())
    }

    fn part_one(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        Ok(tally_by(input, |[c1, c2]| {
            game_score(Throw::try_from(c1).unwrap(), Throw::try_from(c2).unwrap())
        })
        .to_string())
    }

    fn part_two(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        Ok(tally_by(input, |[c1, c2]| {
            game_score2(
                Throw::try_from(c1).unwrap(),
                GameResult::try_from(c2).unwrap(),
            )
        })
        .to_string())
    }
}

#[cfg(test)]
mod test {
    mod unit {}

    mod integration {}
}
