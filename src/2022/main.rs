use std::collections::HashMap;
use utils::solution::*;

mod solutions;

fn main() {
    let cli = utils::cli::make_cli();

    let mut solutions: HashMap<i32, Box<dyn Solution>> = HashMap::new();
    solutions.insert(1, Box::new(solutions::day01::Day01 {}));

    let root_dir = cli.value_of("inputs").unwrap();
    for day in utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(solution) = solutions.get(&day) {
            execute_with_timing(day, &format!("{}/day{:02}.txt", root_dir, day), solution)
        }
    }
}
