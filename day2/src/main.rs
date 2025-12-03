use std::fmt;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::ops::RangeInclusive;

fn read_stdin() -> String {
    io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid range format"),
            ParseError::InvalidNumber(e) => write!(f, "Invalid number: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::InvalidNumber(err)
    }
}

fn parse_input() -> Result<Vec<RangeInclusive<u64>>, ParseError> {
    read_stdin()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').ok_or(ParseError::InvalidFormat)?;
            let start = a.parse::<u64>()?;
            let end = b.parse::<u64>()?;
            Ok(start..=end)
        })
        .collect()
}

fn is_valid_id_part1(id: &u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 == 0 {
        let mid = len / 2;
        let first_half = &s[..mid];
        let second_half = &s[mid..];

        if first_half == second_half {
            return false;
        }
    }

    true
}

fn is_valid_id_part2(id: &u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[..pattern_len];
            let repetitions = len / pattern_len;

            if pattern.repeat(repetitions) == s {
                return false;
            }
        }
    }

    true
}

fn main() {
    let ranges = parse_input().expect("Failed to parse input");

    let invalid_ids_part1: Vec<u64> = ranges.clone()
        .into_iter()
        .flatten()
        .filter(|id| !is_valid_id_part1(id))
        .collect();
    println!("Invalid IDs for Part 1: {:?}", invalid_ids_part1);

    let solution_part1: u64 = invalid_ids_part1
        .iter()
        .sum();
    println!("Part 1 Solution: {}", solution_part1);

    let invalid_ids_part2: Vec<u64> = ranges.clone()
        .into_iter()
        .flatten()
        .filter(|id| !is_valid_id_part2(id))
        .collect();
    println!("Invalid IDs for Part 2: {:?}", invalid_ids_part2);

    let solution_part2: u64 = invalid_ids_part2
        .iter()
        .sum();
    println!("Part 2 Solution: {}", solution_part2);
}
