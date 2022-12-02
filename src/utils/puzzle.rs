use crate::error::Error;

// A specialized result type for solving Advent of Code challenges.
pub type PuzzleResult = [Result<String, Error>; 2];

pub trait Puzzle {
    fn solution(&self, input: &str) -> PuzzleResult;
}

impl<T: PuzzleImpl> Puzzle for T {
    fn solution(&self, input: &str) -> PuzzleResult {
        let parsed_input = Self::parse_input(input).expect("Could not parse input");
        [Self::part_one(&parsed_input), Self::part_two(&parsed_input)]
    }
}

pub trait PuzzleImpl {
    type ParsedInput;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, Error>;

    fn part_one(parsed_input: &Self::ParsedInput) -> Result<String, Error>;
    fn part_two(parsed_input: &Self::ParsedInput) -> Result<String, Error>;
}

pub fn execute_with_timing<P>(
    day: i32,
    input_file: &P,
    puzzle: &Box<dyn Puzzle>,
) -> std::result::Result<(), std::io::Error>
where
    P: std::fmt::Debug + std::convert::AsRef<std::path::Path>,
{
    let input = std::fs::read_to_string(input_file)?;

    let now = std::time::Instant::now();
    let result = puzzle.solution(&input);
    let elapsed_time = now.elapsed().as_micros();

    println!("Day {0:02} ({2:>7}): {1:?}", day, result, elapsed_time);

    Ok(())
}
