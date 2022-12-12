use crate::error::Error;

pub fn read_input_file(year: i32, day: i32) -> Result<String, Error> {
    let fname = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(year.to_string())
        .join("inputs")
        .join(format!("day{:02}.txt", day));

    std::fs::read_to_string(&fname).map_err(|_| Error::InvalidFile(fname.clone()))
}

#[macro_export]
macro_rules! verify_part {
    ($puzzle:ident::$part:ident, $year:expr, $day:expr, $answer:expr) => {
        let input = utils::verification::read_input_file($year, $day).unwrap();
        let parsed_input = &$puzzle::parse_input(&input).unwrap();
        let result = $puzzle::$part(parsed_input).unwrap();

        assert_eq!(
            result, $answer,
            "Expected result to match answer submitted to Advent of Code"
        );
    };
}

#[macro_export]
macro_rules! verify_submitted_answers {
    ($puzzle:ident, $year:expr, $day:expr, $answer_a:expr, $answer_b:expr) => {
        mod verification {
            use super::*;

            #[test]
            fn part_a() {
                utils::verify_part!($puzzle::part_a, $year, $day, $answer_a);
            }

            #[test]
            fn part_b() {
                utils::verify_part!($puzzle::part_b, $year, $day, $answer_b);
            }
        }
    };
}
