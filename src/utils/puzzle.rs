use crate::error::Error;

pub type PartResult = Result<String, Error>;

// A specialized result type for solving Advent of Code challenges.
pub type PuzzleResult = Result<(PartResult, PartResult), Error>;

pub trait HasPuzzleData<T> {
    fn get_data(&self) -> T;
    fn set_data(&self, data: &T);
}

pub trait Puzzle {
    fn solution(&self, input: &str) -> PuzzleResult;
}

impl<T: PuzzleImpl> Puzzle for T {
    fn solution(&self, input: &str) -> PuzzleResult {
        let parsed_input = Self::parse_input(input)?;
        Ok((Self::part_a(&parsed_input), Self::part_b(&parsed_input)))
    }
}

pub trait PuzzleImpl {
    type ParsedInput;

    fn parse_input(input: &str) -> Result<Self::ParsedInput, Error>;

    fn part_a(parsed_input: &Self::ParsedInput) -> PartResult;
    fn part_b(parsed_input: &Self::ParsedInput) -> PartResult;
}

pub struct TimedResults {
    part_a_result: PartResult,
    part_b_result: PartResult,
    elapsed_time: u128,
}

impl std::fmt::Display for TimedResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>7} ({:?}, {:?})",
            self.elapsed_time, self.part_a_result, self.part_b_result
        )
    }
}

pub fn execute_with_timing<P: Puzzle>(
    input_file: &std::path::Path,
    puzzle: P,
) -> std::result::Result<TimedResults, Error>
where
    P: std::fmt::Debug + std::convert::AsRef<std::path::Path>,
{
    let input = std::fs::read_to_string(input_file)?;

    let now = std::time::Instant::now();
    let (part_a_result, part_b_result) = puzzle.solution(&input)?;
    let elapsed_time = now.elapsed().as_micros();

    Ok(TimedResults {
        part_a_result,
        part_b_result,
        elapsed_time,
    })
}
