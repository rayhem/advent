use core::time;
use std::collections::HashMap;
use utils::puzzle::*;

mod solutions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = utils::cli::make_cli();

    let mut puzzles: HashMap<i32, Box<dyn Puzzle>> = HashMap::new();
    puzzles.insert(1, Box::new(solutions::day01::Day01 {}));
    puzzles.insert(2, Box::new(solutions::day02::Day02 {}));
    puzzles.insert(3, Box::new(solutions::day03::Day03 {}));

    let root_dir = cli.value_of("inputs").unwrap();
    for day in utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(puzzle) = puzzles.get(&day) {
            let filename = std::path::Path::from(format!("{}/day{:02}.txt", root_dir, day));
            let timed_results = execute_with_timing(&filename, puzzle)?;

            println!("Day {:2>}: {}", day, timed_results);
        }
    }

    Ok(())
}
