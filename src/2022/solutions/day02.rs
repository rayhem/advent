use std::str::FromStr;
use utils::puzzle::PuzzleImpl;

pub struct GameRound {
    // The prescription for what the opponent will play in one round of the
    // Elves' game of Rock, Paper, Scissors. This is given by the first
    // character (of two) and always denotes the actual throw the player should
    // make.
    opponent_play: i32,

    // The prescription for what _you_ should play in one round of the Elves'
    // game of Rock, Paper, Scissors. This is given by the second character (of
    // two) and has a different meaning depending on the part of the puzzle. In
    // part one, this behaves the same as the opponent's play: it maps directly
    // to the throw to make. In part two, it indicates the outcome of the game
    // instead.
    self_play: i32,
}

impl GameRound {
    fn part1_score(&self) -> i32 {
        let player = self.self_play;
        let outcome = (player - self.opponent_play + 1).wrapping_rem_euclid(3);
        (player + 1) + 3 * outcome
    }

    fn part2_score(&self) -> i32 {
        let outcome = self.self_play;
        let player = (outcome + self.opponent_play - 1).wrapping_rem_euclid(3);
        (player + 1) + 3 * outcome
    }
}

impl std::str::FromStr for GameRound {
    type Err = utils::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c1 = chars.next().ok_or(Self::Err::ParseError)?;
        let opponent_play = (c1 as i32) - ('A' as i32);

        let c2 = chars.nth(1).ok_or(Self::Err::ParseError)?;
        let self_play = (c2 as i32) - ('X' as i32);

        Ok(Self {
            opponent_play,
            self_play,
        })
    }
}

pub struct Day02 {}

impl PuzzleImpl for Day02 {
    type ParsedInput = Vec<GameRound>;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, utils::error::Error> {
        Ok(input.lines().flat_map(FromStr::from_str).collect())
    }

    fn part_one(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        Ok(input
            .iter()
            .map(GameRound::part1_score)
            .sum::<i32>()
            .to_string())
    }

    fn part_two(input: &Self::ParsedInput) -> Result<String, utils::error::Error> {
        Ok(input
            .iter()
            .map(GameRound::part2_score)
            .sum::<i32>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    mod unit {}

    mod integration {}
}
